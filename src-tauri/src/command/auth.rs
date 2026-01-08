use std::time::Duration;

use tower::common::bincode_encode;
use tower::common::user::{ChangePasswordReq, LoginReq, LoginResp, RegisterReq, UserInfoResp};
use tower::common::ApiError;
use tower::common::App;
use tower::common::ConfigGetter;
use tower::common::StoreKey;
use tower::common::TowerResource;
use tower::common::Urlable;
use tower::config_model::Config;
use tower::jwt::JwtString;
use tower::jwt::Principal;
use tower::jwt::Token;
use tower::jwt::TokenError;
use tower::reqwest::async_post_bin_and;
use tracing::debug;

use crate::common::get_token;
use crate::common::refresh_token;
use crate::common::set_refresh_token;
use crate::common::set_token;
use crate::common::{delete_store, get_user_store_name};

#[tauri::command]
pub async fn get_login_info(
    config: tauri::State<'_, Config>,
    webview: tauri::WebviewWindow,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, principal) = get_login_user(&webview, &tower_server).await?;
    Ok(bincode_encode(UserInfoResp {
        user_id: principal.user_id,
        nickname: principal.nickname,
        avatar: principal.avatar,
    }))
}

pub async fn get_login_user(
    webview: &tauri::WebviewWindow,
    tower_server: &str,
) -> Result<(String, Principal), ApiError> {
    //获取access_token
    let token = get_token(webview)?;
    //获取用户信息
    let res: Result<Principal, TokenError> = Token(token.clone()).get_principal();

    debug!("get_login_user:{:?}", res);
    match res {
        Ok(user) => Ok((token, user)),
        Err(err) => match err {
            TokenError::Invalid => Err(ApiError::UnAuthorite),
            TokenError::Expired => refresh_token(webview, tower_server, token).await,
        },
    }
}

#[tauri::command]
pub async fn login(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    mut req: LoginReq,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let url = TowerResource::Auth.url(tower_server, "/login");
    req.password = JwtString(req.password).signature(Duration::from_secs(5));
    req.device_code = JwtString(get_user_store_name(&webview)).signature(Duration::from_secs(5));

    let resp: Result<LoginResp, ApiError> = async_post_bin_and(&url, &req, None).await;
    match resp {
        Ok(resp) => {
            set_token(&webview, resp.access)?;
            set_refresh_token(&webview, resp.refresh)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn register(
    config: tauri::State<'_, Config>,
    mut req: RegisterReq,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    req.password = JwtString(req.password).signature(Duration::from_secs(5));

    let url = TowerResource::Auth.url(tower_server, "/register");
    let resp: Result<(), ApiError> = async_post_bin_and(&url, &req, None).await;
    resp
}

#[tauri::command]
pub async fn change_password(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    mut req: ChangePasswordReq,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    req.password = JwtString(req.password).signature(Duration::from_secs(5));
    req.new_password = JwtString(req.new_password).signature(Duration::from_secs(5));

    let (token, _) = get_login_user(&webview, &tower_server).await?;
    let url = TowerResource::Auth.url(tower_server, "/change_password");
    let resp: Result<(), ApiError> = async_post_bin_and(&url, &req, Some(token)).await;
    resp
}

#[tauri::command]
pub async fn logout(webview: tauri::WebviewWindow) -> Result<(), ApiError> {
    delete_store(&webview, StoreKey::AccessToken)?;
    delete_store(&webview, StoreKey::RefreshToken)?;
    // delete_store(&webview, StoreKey::Hotkey)?;

    Ok(())
}
