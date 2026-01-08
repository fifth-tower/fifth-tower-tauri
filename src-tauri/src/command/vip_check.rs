use std::collections::HashMap;

use flow::{record::get_flow_num, Dir};
use tower::{
    common::{
        dict::{DictData, VipLevelItem},
        ApiError, ApiMethod, ConfigGetter, Urlable,
    },
    config_model::Config,
    reqwest::async_get_and,
    script_model::TowerScriptResource,
};

use crate::command::auth::get_login_user;

#[tauri::command]
pub async fn check_flow_num(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    dict: tauri::State<'_, DictData>,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(tower::common::App::TowerServer);
    let current_num = get_flow_num(&Dir::Record);
    check_vip_level_item(
        &webview,
        &dict,
        &tower_server,
        &VipLevelItem::RecordFlowNum,
        current_num,
    )
    .await
}

#[tauri::command]
pub async fn check_wishing_num(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    dict: tauri::State<'_, DictData>,
) -> Result<(), ApiError> {
    let (tower_server, tower_assitant_server) = (
        config.get_string(tower::common::App::TowerServer),
        config.get_string(tower::common::App::TowerScriptServer),
    );
    let (token, _) = get_login_user(&webview, &tower_server).await?;
    let current_num = async_get_and(
        &TowerScriptResource::WishingWall.url(&tower_assitant_server, ApiMethod::Count),
        &HashMap::<String, String>::new(),
        Some(token),
    )
    .await?;
    check_vip_level_item(
        &webview,
        &dict,
        &tower_assitant_server,
        &VipLevelItem::WishingNum,
        current_num,
    )
    .await
}

#[tauri::command]
pub async fn get_vip_level_item_num(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    dict: tauri::State<'_, DictData>,
    item: VipLevelItem,
) -> Result<usize, ApiError> {
    let tower_server = config.get_string(tower::common::App::TowerServer);
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;
    let dicts = dict.get_dict(&login_user.vip_level);
    Ok(item.val(&dicts))
}

///current >= config_num
pub async fn check_vip_level_item(
    webview: &tauri::WebviewWindow,
    dict: &DictData,
    tower_server: &str,
    item: &VipLevelItem,
    current: usize,
) -> Result<(), ApiError> {
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;
    let dicts = dict.get_dict(&login_user.vip_level);
    let config_num = item.val(&dicts);
    if current >= config_num {
        Err(item.error(config_num))
    } else {
        Ok(())
    }
}
