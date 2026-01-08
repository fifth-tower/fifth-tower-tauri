/// 秘籍的录制、测试运行
use std::sync::Mutex;

use flow::Recording;
use flow_model::{ActionCommand, ActionConfig, RecordType};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::*;

use tauri_plugin_dialog::*;
use tower::common::{bincode_dec, bincode_encode, StoreKey};
use tracing::debug;

use crate::common::{get_mouse_postion, get_store_value, set_store_value, FunctionId};

use super::ActionRecorder;

#[tauri::command]
pub fn start_record(
    app: AppHandle,
    webview: tauri::WebviewWindow,
    recording: tauri::State<Mutex<Recording>>,
    pid: u32,
    app_id: Option<&str>,
    flow_id: Option<&str>,
    record_type: RecordType,
) -> (String, String, Vec<(String, (ActionCommand, bool))>) {
    debug!("start_record:{}, {:?}, {:?}", pid, app_id, flow_id);
    let shortcut_map = get_shortcut_map(&app, record_type);
    //取消快捷键注册
    let _ = app
        .global_shortcut()
        .unregister_multiple(shortcut_map.iter().map(|f| f.0));

    //重新注册
    let mut recording = recording.lock().unwrap();
    recording.start(pid, app_id, flow_id);

    let log_handle_fn = move |message: ActionConfig| {
        let message = bincode_encode(message);
        FunctionId::Record.call_func(&webview, &message);
    };
    let mut maps = vec![];
    let mut record_status = true;
    for map in shortcut_map {
        let map1 = map.1.clone();
        let app_id = recording.app_id();
        let flow_id = recording.flow_id();
        let log_handle_fn = log_handle_fn.clone();
        let ret =
            app.global_shortcut()
                .on_shortcut(map.0, move |app, _shortcut, event| match event.state() {
                    ShortcutState::Pressed => {
                        map1.fill_command(&app_id, &flow_id, get_mouse_postion())
                            .map(|action| {
                                let is_ok = app
                                    .dialog()
                                    .message(action.message(true))
                                    .title("动作录制")
                                    .buttons(MessageDialogButtons::OkCancelCustom(
                                        "确定".to_string(),
                                        "取消".to_string(),
                                    ))
                                    .blocking_show();
                                if is_ok {
                                    let state = app.state::<Mutex<Recording>>();

                                    let mut recording = state.lock().unwrap();
                                    let ac = recording.add_action(action);
                                    log_handle_fn(ac);
                                }
                            });
                    }
                    ShortcutState::Released => {}
                });

        record_status &= ret.is_ok();
        maps.push((map.0.into_string(), (map.1, ret.is_ok())));
    }
    let ret = (recording.app_id(), recording.flow_id(), maps);
    //register shortcut fail ,cancel recording
    if (!record_status) {
        let _ = app
            .global_shortcut()
            .unregister_multiple(get_shortcut_map(&app, record_type).iter().map(|f| f.0));

        match record_type {
            RecordType::Flow => recording.end(),
            RecordType::Action(_) => recording.reset(),
        };
    }
    ret
}

#[tauri::command]
pub fn end_record(
    app: AppHandle,
    recording: tauri::State<Mutex<Recording>>,
    record_type: RecordType,
) {
    debug!("End recording: {:?}", record_type);
    let _ = app
        .global_shortcut()
        .unregister_multiple(get_shortcut_map(&app, record_type).iter().map(|f| f.0));

    let mut recording = recording.lock().unwrap();

    match record_type {
        RecordType::Flow => recording.end(),
        RecordType::Action(_) => recording.reset(),
    };
}

#[tauri::command]
pub fn cancel_record(app: AppHandle, recording: tauri::State<Mutex<Recording>>) {
    let _ = app
        .global_shortcut()
        .unregister_multiple(get_shortcut_map(&app, RecordType::Flow).iter().map(|f| f.0));

    let mut recording = recording.lock().unwrap();
    recording.cancel();
}

#[tauri::command]
pub fn set_hotkeys(
    app: AppHandle,
    hotkeys: Vec<(Shortcut, ActionCommand)>,
) -> Vec<(String, (ActionCommand, bool))> {
    let _ = app
        .global_shortcut()
        .unregister_multiple(get_shortcut_map(&app, RecordType::Flow).iter().map(|f| f.0));

    let mut maps = vec![];
    let mut is_ok = true;
    for map in &hotkeys {
        let ret =
            app.global_shortcut()
                .on_shortcut(map.0, move |_app, _shortcut, event| match event.state() {
                    ShortcutState::Pressed => {
                        debug!("set shortcut success");
                    }
                    ShortcutState::Released => {}
                });

        is_ok &= ret.is_ok();
        maps.push((map.0.into_string(), (map.1.clone(), ret.is_ok())));
    }
    let _ = app
        .global_shortcut()
        .unregister_multiple(hotkeys.iter().map(|f| f.0));
    if is_ok {
        set_store_value(&app, StoreKey::Hotkey, bincode_encode(hotkeys)).unwrap();
    }
    maps
}

fn get_shortcut_map(app: &AppHandle, record_type: RecordType) -> Vec<(Shortcut, ActionCommand)> {
    let hotkeys = get_store_value(app, StoreKey::Hotkey);
    hotkeys
        .map(|hotkey| bincode_dec(&hotkey).ok())
        .flatten()
        .unwrap_or(
            vec![
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyK),
                    ActionCommand::of("Move"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyL),
                    ActionCommand::of("Click"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyI),
                    ActionCommand::of("Image"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyJ),
                    ActionCommand::of("Scroll"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyG),
                    ActionCommand::of("GuaGuaLe"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyO),
                    ActionCommand::of("Input"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyH),
                    ActionCommand::of("KeyCombi"),
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyP),
                    ActionCommand::IncludeFlow,
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN),
                    ActionCommand::SubFlowStart,
                ),
                (
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyM),
                    ActionCommand::SubFlowEnd,
                ),
            ]
            .into_iter()
            .filter(|(_, command)| match record_type {
                RecordType::Flow => true,
                RecordType::Action(_) => !matches!(
                    command,
                    ActionCommand::SubFlowStart | ActionCommand::SubFlowEnd
                ),
            })
            .collect(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
