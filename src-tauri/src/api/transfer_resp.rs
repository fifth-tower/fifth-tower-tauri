use bytes::Bytes;
use serde::de::DeserializeOwned;
use tauri::http::StatusCode;
use tauri_plugin_http::reqwest::{Error, Response};
use tower::common::{bincode_dec, ApiError};
use tracing::{error, info};

use crate::common::FunctionId;

pub trait TransferReqwestResponse {
    async fn transfer<T: DeserializeOwned>(
        self,
        webview: &tauri::WebviewWindow,
    ) -> Result<T, ApiError>;

    async fn transfer_text(self, webview: &tauri::WebviewWindow) -> Result<String, ApiError>;

    async fn transfer_bytes(self, webview: &tauri::WebviewWindow) -> Result<Bytes, ApiError>;
}

pub type ReqwestResponse = Result<Response, Error>;

impl TransferReqwestResponse for ReqwestResponse {
    async fn transfer<T: DeserializeOwned>(
        self,
        webview: &tauri::WebviewWindow,
    ) -> Result<T, ApiError> {
        self.transfer_text(webview).await.and_then(|f| {
            bincode_dec(&f).map_err(|_| {
                error!("tauri解码时发生异常,body={}", f);
                ApiError::Decoded
            })
        })
    }

    async fn transfer_text(self, webview: &tauri::WebviewWindow) -> Result<String, ApiError> {
        self.transfer_bytes(webview).await.and_then(|f| {
            let text = String::from_utf8_lossy(&f);
            Ok(text.to_string())
        })
    }
    async fn transfer_bytes(self, webview: &tauri::WebviewWindow) -> Result<Bytes, ApiError> {
        match self {
            Ok(resp) => match resp.status() {
                StatusCode::UNAUTHORIZED => {
                    FunctionId::Login.call_func(webview, "refresh");
                    Err(ApiError::UnAuthorite)
                }
                StatusCode::FORBIDDEN => Err(ApiError::Forbidden),
                StatusCode::OK => {
                    let body = resp.bytes().await.unwrap();
                    Ok(body)
                }
                _ => {
                    let body = resp.text().await.unwrap();
                    error!("tauri请求时，服务端发生异常,resp={:?}", body);
                    Err(ApiError::ServerError)
                }
            },
            Err(err) => {
                error!("web请求时发生异常，err={:?}", err);
                Err(ApiError::BadClient)
            }
        }
    }
}
