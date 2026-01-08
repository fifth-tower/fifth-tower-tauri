use std::collections::HashMap;
use std::path::Path;

use assitant_model::script::InstallScriptItem;
use flow::{record, script, Dir};
use flow_model::{Action, Actions, Tag, ZipTagType};
use fs_extra::{dir, remove_items};
use multimap::MultiMap;
use tower::common::bincode_encode;
use tower::common::social::{GetReportsByIdsReq, GetReportsByIdsResp, TargetType};
use tower::common::ApiError;
use tower::common::ApiMethod;
use tower::common::ApiResponse;
use tower::common::ConfigGetter;
use tower::common::TowerResource;
use tower::common::{App, Urlable};
use tower::common_rs::{as_zip_password, unzip_with_filter, zip_dir};
use tower::config_model::Config;
use tower::reqwest;
use tower::script_model::ScriptUploadReq;
use tower::script_model::TowerScriptResource;

use crate::api::{self, TransferReqwestResponse};

use super::auth::get_login_user;

#[tauri::command]
pub async fn upload_script(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    mut req: ScriptUploadReq,
) -> Result<ApiResponse<()>, ApiError> {
    dir::create_all(Dir::Tmp.path(), true).unwrap();
    let tower_server = config.get_string(App::TowerServer);
    let (token, login_user) = get_login_user(&webview, &tower_server).await?;

    //更新本地秘籍版本
    let ScriptUploadReq {
        app_id,
        zip_name,
        description,
        version,
        fee,
        ..
    } = &req;
    record::update_app_with_zip(&Dir::Record, app_id, |app| {
        app.version = version.into();
        app.zip_name = Some(zip_name.into());
        app.zip_description = Some(description.into());
        app.fee = *fee;
        if app.tags.is_empty() {
            app.tags = vec![Tag::new(
                &login_user.user_id,
                &login_user.nickname,
                &ZipTagType::User,
            )];
            Ok(())
        } else {
            for tag in &app.tags {
                match tag.typ {
                    ZipTagType::App => {}
                    ZipTagType::Version => {}
                    ZipTagType::User => {
                        if tag.id.ne(&login_user.user_id) {
                            return Err("您不是秘籍作者，不可以上传。".into());
                        }
                    }
                    ZipTagType::Org => todo!(),
                }
            }
            Ok(())
        }
    })
    .map_err(|msg| ApiError::Custom(msg))?;

    //压缩目录
    //需要压缩的图片
    let zip_images = get_need_images(app_id);
    let zip_path = Dir::Tmp.app(app_id);
    let zip_path = Path::new(&zip_path);
    zip_dir(
        Path::new(Dir::Record.app(app_id).as_str()),
        zip_path,
        Some(&as_zip_password(&login_user.user_id)),
        |path| need_zip(path, &zip_images),
    );
    //上传zip
    req.user_name = Some(login_user.nickname);
    req.user_id = Some(login_user.user_id);

    let url = TowerScriptResource::Script.url(config.get_string(App::TowerScriptServer), "/upload");
    let resp = api::upload_script(url, &zip_path, &token, &req).await;

    dir::create(Dir::Tmp.path(), true).unwrap();

    resp.transfer(&webview).await
}

///获取需要打包的图片
fn get_need_images(app_id: &str) -> Vec<String> {
    fn flow_func(one: &mut Vec<String>, actions: &Actions) {
        for (action, ..) in actions {
            if let Action::Image(_, template, ..) = action {
                one.push(template.clone());
            }
            if let Action::SubFlow(sub_actions, ..) = action {
                flow_func(one, sub_actions);
            }
        }
    }
    let app = record::load_app(&Dir::Record, app_id);
    let mut images = Vec::new();
    for flow in &app.flows {
        flow_func(&mut images, &flow.actions);
    }
    images
}

///判断文件是否需要打包
fn need_zip(path: &Path, zip_images: &Vec<String>) -> bool {
    //allow app.setting and *.flow
    let allow_flow = path
        .file_name()
        .map(|name| name.to_str())
        .flatten()
        .map_or(false, |name| {
            Dir::is_app_file(name) || Dir::is_flow_file(name)
        });
    if allow_flow {
        return true;
    }
    let allow_image = zip_images.iter().any(|image| path.ends_with(image));
    if allow_image {
        return true;
    }
    false
}
///删除秘籍
#[tauri::command]
pub async fn delete_script(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    zip_id: &str,
    app_id: &str,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;

    let script_dir = Dir::Script(login_user.user_id, zip_id.to_owned());
    let zip_file = script_dir.app(app_id);
    let zip_reources = script_dir.image(app_id, None).0;

    remove_items(&vec![zip_file, zip_reources]).unwrap();
    Ok(())
}

///下载秘籍
#[tauri::command]
pub async fn download_script(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    zip_id: &str,
    app_id: &str,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (token, login_user) = get_login_user(&webview, &tower_server).await?;

    let url =
        TowerScriptResource::Script.url(config.get_string(App::TowerScriptServer), "/download");
    let resp = api::download_script(url, &token, zip_id).await;
    let resp = resp.transfer_bytes(&webview).await?;

    //save zip
    let dir = Dir::Script(login_user.user_id.clone(), zip_id.into());
    let file_path = dir.app(app_id);
    tower::common_rs::bytes_to_file(&file_path, &resp, false);

    //unzip res
    unzip_with_filter(
        &file_path,
        &format!("{}/{}/{}", dir.path(), app_id, zip_id),
        Some(&as_zip_password(&login_user.user_id)),
        |path: &Path| {
            path.file_name()
                .map(|name| name.to_str())
                .flatten()
                .map_or(false, |name| {
                    Dir::is_app_file(name) || Dir::is_flow_file(name)
                })
        },
        false,
    );
    Ok(())
}

///获取我的秘籍列表
#[tauri::command]
pub async fn get_installed_scripts(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;
    let ret = script::get_installed_zips(&Dir::Script(login_user.user_id, "".into()));
    let zip_ids: Vec<String> = ret.iter().map(|(zip_id, _)| zip_id.clone()).collect();

    let url = TowerResource::Report.url(config.get_string(App::TowerServer), ApiMethod::List);

    let reports: HashMap<String, GetReportsByIdsResp> = reqwest::async_post_and(
        &url,
        &GetReportsByIdsReq {
            target_ids: zip_ids,
            target_type: TargetType::AssitantZip.into(),
        },
        None,
    )
    .await
    .unwrap_or_default();

    let ret: MultiMap<String, InstallScriptItem> = ret
        .into_iter()
        .map(|(zip_id, app_info)| {
            (
                app_info.app_name.clone(),
                InstallScriptItem {
                    score: reports.get(&zip_id).map_or(0, |resp| resp.avarage_score()),
                    zip_id,
                    app_info,
                },
            )
        })
        .collect();
    Ok(bincode_encode(ret))
}

///获取秘籍appinfo
/// return (zip_id,app_info)
#[tauri::command]
pub async fn get_scripts_info(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    //app_id, zip_id
    ids: Vec<(String, String)>,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;
    let ret = script::load_apps_with_flows_for_web(
        &ids.into_iter()
            .map(|(app_id, zid)| (app_id, Dir::Script(login_user.user_id.clone(), zid)))
            .collect(),
    );
    Ok(ret)
}
