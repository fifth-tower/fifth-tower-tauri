use std::collections::HashMap;

use crate::command::auth::get_login_user;
use crate::common::FunctionId;

use tower::common::ApiError;
use tower::common::App;
use tower::common::ConfigGetter;
use tower::config_model::Config;
use tower::reqwest::delete_bin_text;
use tower::reqwest::post_bin_text;
use tower::reqwest::put_bin_text;
use tower::reqwest::{delete_text, get, post_text, put_text, TransferResponse};
use tracing::debug;

#[tauri::command]
pub async fn http_bin(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    app_name: &str,
    method: &str,
    url: &str,
    req: String,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (token, _) = get_login_user(&webview, &tower_server).await?;

    let url = format!("{}{}", config.get_string(app_name), url);

    debug!("token:{}", token);
    let ret = send_bin(method, &url, req, Some(token)).await;
    if let Err(ApiError::UnAuthorite) = ret {
        FunctionId::Login.call_func(&webview, "refresh");
    }
    ret
}

#[tauri::command]
pub async fn http_bin_without_token(
    config: tauri::State<'_, Config>,
    app_name: &str,
    method: &str,
    url: &str,
    req: String,
) -> Result<String, ApiError> {
    let url = format!("{}{}", config.get_string(app_name), url);
    send_bin(method, &url, req, None).await
}

#[tauri::command]
pub async fn http(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    app_name: &str,
    method: &str,
    url: &str,
    req: String,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (token, _) = get_login_user(&webview, &tower_server).await?;

    let url = format!("{}{}", config.get_string(app_name), url);

    debug!("token:{}", token);
    let ret = send(method, &url, req, Some(token)).await;
    if let Err(ApiError::UnAuthorite) = ret {
        FunctionId::Login.call_func(&webview, "refresh");
    }
    ret
}

#[tauri::command]
pub async fn http_without_token(
    config: tauri::State<'_, Config>,
    app_name: &str,
    method: &str,
    url: &str,
    req: String,
) -> Result<String, ApiError> {
    let url = format!("{}{}", config.get_string(app_name), url);
    send(method, &url, req, None).await
}

pub(crate) async fn send(
    method: &str,
    url: &str,
    req: String,
    token: Option<String>,
) -> Result<String, ApiError> {
    debug!("{} {}, req:{}", method, url, req);
    let resp = match method.to_lowercase().as_str() {
        "get" => {
            let req: HashMap<String, String> = serde_json::from_str(&req).unwrap();
            get(&url, &req, token).await
        }
        "post" => post_text(&url, req, token).await,
        "put" => put_text(&url, req, token).await,
        "delete" => delete_text(&url, req, token).await,
        _ => return Err(ApiError::Custom(format!("method:{} unsupported", method))),
    };
    resp.transfer_text().await
}

pub(crate) async fn send_bin(
    method: &str,
    url: &str,
    req: String,
    token: Option<String>,
) -> Result<String, ApiError> {
    debug!("{} {}, req:{}", method, url, req);
    let resp = match method.to_lowercase().as_str() {
        "post" => post_bin_text(&url, req, token).await,
        "put" => put_bin_text(&url, req, token).await,
        "delete" => delete_bin_text(&url, req, token).await,
        _ => return Err(ApiError::Custom(format!("method:{} unsupported", method))),
    };
    resp.transfer_text().await
}
