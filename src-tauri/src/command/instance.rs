use std::{sync::Arc, time::Duration};

use assitant_model::{record::CheckPoint, script::ExecuteScriptsKey};
use flow::{
    get_window, is_same_size, record, script, sync_sleep, Dir, Executable, FLowResource,
    FlowExecutor,
};
use flow_model::{ActionConfig, CustomAction, Flow, FlowInfo, LogContent, LogMessage, RunStatus};
use multimap::MultiMap;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::*;
use tokio::sync::Barrier;
use tower::common::dict::{DictData, VipLevelItem};
use tower::common::ApiError;
use tower::common::{bincode_decode, bincode_encode, ConfigGetter};
use tower::config_model::Config;
use tracing::{debug, error};

use crate::common::FunctionId;

use super::auth::get_login_user;

///终止运行
#[tauri::command]
pub fn stop_instance(
    app_handle: AppHandle,
    resource: tauri::State<Arc<FLowResource>>,
    pids: Vec<u32>,
) {
    let _ = app_handle
        .global_shortcut()
        .unregister(Shortcut::new(Some(Modifiers::CONTROL), Code::KeyZ));

    for pid in pids {
        resource.set_stop(pid);
    }
}

///录制时,流程测试
#[tauri::command]
pub fn test_flow(
    app_handle: AppHandle,
    webview: tauri::WebviewWindow,
    resource: tauri::State<Arc<FLowResource>>,
    pid: u32,
    app_id: &str,
    flow_str: &str,
) {
    let sub_flow: Flow = bincode_decode(flow_str);
    let app = record::load_app(&Dir::Record, app_id);
    //注册快捷键
    register_stop_shortcut(&app_handle, &resource, vec![pid]);
    //执行
    let log_handle_fn = move |message: LogMessage| {
        let need_replace = bincode_encode(message.need_replace());
        let message = bincode_encode(message);
        FunctionId::Test.call_func_2(&webview, &message, &need_replace);
    };
    let resource = Arc::clone(&resource);
    tauri::async_runtime::spawn_blocking(move || {
        let mut executor =
            FlowExecutor::new(pid, app, resource, Dir::Record, Box::new(log_handle_fn));
        sub_flow.execute(&mut executor);

        let _ = app_handle
            .global_shortcut()
            .unregister(Shortcut::new(Some(Modifiers::CONTROL), Code::KeyZ));
    });
}

///录制时,action测试
#[tauri::command]
pub fn test_action_config(
    app_handle: AppHandle,
    webview: tauri::WebviewWindow,
    resource: tauri::State<Arc<FLowResource>>,
    pid: u32,
    app_id: &str,
    action_str: &str,
) {
    let action_config: ActionConfig = bincode_decode(action_str);
    let app = record::load_app(&Dir::Record, app_id);
    //注册快捷键
    register_stop_shortcut(&app_handle, &resource, vec![pid]);
    //执行
    let log_handle_fn = move |message: LogMessage| {
        let need_replace = bincode_encode(message.need_replace());
        let message = bincode_encode(message);
        FunctionId::Test.call_func_2(&webview, &message, &need_replace);
    };
    let resource = Arc::clone(&resource);
    tauri::async_runtime::spawn_blocking(move || {
        let mut executor =
            FlowExecutor::new(pid, app, resource, Dir::Record, Box::new(log_handle_fn));
        action_config.execute(&mut executor);

        let _ = app_handle
            .global_shortcut()
            .unregister(Shortcut::new(Some(Modifiers::CONTROL), Code::KeyZ));
    });
}
///重新注册 全部停止快捷键
fn register_stop_shortcut(
    app_handle: &AppHandle,
    resource: &tauri::State<Arc<FLowResource>>,
    pids: Vec<u32>,
) {
    let _ = app_handle
        .global_shortcut()
        .unregister(Shortcut::new(Some(Modifiers::CONTROL), Code::KeyZ));

    let resource = Arc::clone(resource);
    app_handle
        .global_shortcut()
        .on_shortcut(
            Shortcut::new(Some(Modifiers::CONTROL), Code::KeyZ),
            move |_, _, event| match event.state() {
                ShortcutState::Pressed => {
                    for pid in pids.clone() {
                        resource.set_stop(pid);
                    }
                }
                ShortcutState::Released => {}
            },
        )
        .unwrap();
}
///执行选择秘籍
#[tauri::command]
pub async fn execute_scripts(
    app_handle: AppHandle,
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    dict: tauri::State<'_, DictData>,
    resource: tauri::State<'_, Arc<FLowResource>>,
    ids: String,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(tower::common::App::TowerServer);
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;
    let dict = dict.get_dict(&login_user.vip_level);
    let instant_num = VipLevelItem::InstanceNum.val(&dict);

    let ids: MultiMap<ExecuteScriptsKey, FlowInfo> = bincode_decode(&ids);
    if ids.len() > instant_num {
        return Err(VipLevelItem::InstanceNum.error(instant_num));
    }
    let mut pids = vec![];
    ids.into_iter().for_each(
        |(
            ExecuteScriptsKey {
                pid,
                app_id,
                zip_id,
            },
            flow_infos,
        )| {
            pids.push(pid);

            let user_id = login_user.user_id.clone();
            let resource = Arc::clone(&resource);
            let webview = webview.clone();
            let log_handle_fn = move |message: LogMessage| {
                let status = match message.content {
                    LogContent::Flow(flow_id, _) => (flow_id, RunStatus::Running),
                    LogContent::FlowResult(flow_id, _, ret) => (flow_id, RunStatus::Stopped(ret)),
                    LogContent::Stop => ("".into(), RunStatus::Aborted),
                    _ => ("".into(), RunStatus::Init),
                };
                let status = bincode_encode(status);
                FunctionId::Script.call_func_with_pid(&webview, pid, &status);
            };
            tauri::async_runtime::spawn_blocking(move || {
                let dir = Dir::Script(user_id.clone(), zip_id.clone());
                let app = script::load_app(&dir, &app_id, false);
                if app.is_none() {
                    error!("load app {} failed", app_id);
                    return;
                }
                let app = app.unwrap();
                let mut executor =
                    FlowExecutor::new(pid, app, resource, dir.clone(), Box::new(log_handle_fn));

                flow_infos.into_iter().for_each(|FlowInfo { flow_id, .. }| {
                    let flow = script::load_flow(&dir, &app_id, &flow_id);
                    if flow.is_none() {
                        error!("load flow {} failed", flow_id);
                        return;
                    }
                    let flow = flow.unwrap();
                    flow.execute(&mut executor);
                });
            });
        },
    );
    //注册快捷键
    register_stop_shortcut(&app_handle, &resource, pids);
    Ok(())
}

///检查应用窗口
#[tauri::command]
pub async fn check_window(
    webview: tauri::WebviewWindow,
    resource: tauri::State<'_, Arc<FLowResource>>,
    app_id: &str,
    pid: u32,
) -> Result<bool, ()> {
    let resource = Arc::clone(&resource);
    let app_id = app_id.to_owned();

    tauri::async_runtime::spawn_blocking(move || {
        let log = |point: CheckPoint, status: RunStatus, finished: Option<bool>| {
            FunctionId::CheckPoint.call_func(&webview, &bincode_encode((point, status, finished)));
        };

        log(CheckPoint::AppClosed, RunStatus::Init, None);
        let win = get_window(pid);

        //01 检查应用是否关闭
        sync_sleep(Duration::from_secs(1));
        if win.is_none() {
            log(
                CheckPoint::AppClosed,
                RunStatus::Stopped(false),
                Some(false),
            );
            return;
        }
        log(CheckPoint::AppClosed, RunStatus::Stopped(true), None);

        let win = win.unwrap();
        //02 检查应用是否最小化
        log(CheckPoint::Minimized, RunStatus::Init, None);
        sync_sleep(Duration::from_secs(1));

        if win.is_minimized().is_ok_and(|a| a) {
            log(
                CheckPoint::Minimized,
                RunStatus::Stopped(false),
                Some(false),
            );
            return;
        }
        log(CheckPoint::Minimized, RunStatus::Stopped(true), None);

        //03 检查应用大小
        log(CheckPoint::WindowSize, RunStatus::Init, None);
        sync_sleep(Duration::from_secs(1));

        let (win_width, win_height) = (win.width().unwrap(), win.height().unwrap());
        let app_setting = record::load_app_setting(&Dir::Record, &app_id);
        if app_setting.is_none() {
            log(CheckPoint::WindowSize, RunStatus::Stopped(true), Some(true));
            return;
        }
        let app_setting = app_setting.unwrap();
        let (width, height) = (app_setting.width, app_setting.height);

        debug!(
            "before resize: win_dim:{:?}, app_dim:{:?}",
            (win_width, win_height),
            (width, height)
        );
        if is_same_size(win_width, width) && is_same_size(win_height, height) {
            log(CheckPoint::WindowSize, RunStatus::Stopped(true), Some(true));
            return;
        }
        log(CheckPoint::WindowSize, RunStatus::Stopped(false), None);

        //03-1 调整窗口大小
        log(CheckPoint::AdjustSize, RunStatus::Init, None);
        sync_sleep(Duration::from_secs(1));

        let app = record::load_app(&Dir::Record, &app_id);
        let mut executor = FlowExecutor::new(pid, app, resource, Dir::Record, Box::new(|_| {}));
        let resize_flow: Flow = CustomAction::Resize.into();
        let ret = resize_flow.execute(&mut executor);

        log(CheckPoint::AdjustSize, RunStatus::Stopped(ret), Some(ret));
    });
    Ok(false)
}
