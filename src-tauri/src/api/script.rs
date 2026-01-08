use std::{collections::HashMap, path::Path};

use tauri::utils::mime_type::MimeType;
use tauri_plugin_http::reqwest::{
    multipart::{self, Part},
    Client, Error, Response,
};
use tower::script_model::ScriptUploadReq;

///上传秘籍
pub async fn upload_script(
    url: String,
    zip_path: &Path,
    token: &str,
    req: &ScriptUploadReq,
) -> Result<Response, Error> {
    let form = multipart::Form::new().file("file", zip_path).await.unwrap();
    let metadata = Part::text(serde_json::to_string(req).unwrap())
        .mime_str(&MimeType::Json.to_string())
        .unwrap();
    let form = form.part("metadata", metadata);
    Client::new()
        .post(url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await
}

///下载秘籍
pub async fn download_script(url: String, token: &str, zip_id: &str) -> Result<Response, Error> {
    let mut params = HashMap::new();
    params.insert("zip_id", zip_id);
    Client::new()
        .get(url)
        .query(&params)
        .bearer_auth(token)
        .send()
        .await
}
