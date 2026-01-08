use assitant_model::KeyValue;
use enigo::Key;
use tauri_plugin_opener::OpenerExt;
use tower::tauri_model::AssitantConfigKey;
use tower::tauri_model::PublicAsset;
use tower::common::bincode_encode;
use tower::common::ApiError;
use tower::common::App;
use tower::common::ConfigGetter;
use tower::config_model::Config;

use crate::common::constant::VERSION;

#[tauri::command]
pub fn get_control_keys() -> Result<String, ApiError> {
    let keys = vec![Key::Control, Key::Alt, Key::Shift, Key::Tab, Key::Escape];
    let ret: Vec<KeyValue> = keys
        .iter()
        .map(|k| KeyValue {
            name: format!("{:?}", k),
            value: bincode_encode(k),
        })
        .collect();
    Ok(bincode_encode(&ret))
}

//获取公共资产（app_dir，box-server）
#[tauri::command]
pub fn get_public_asset(config: tauri::State<Config>) -> Result<String, ApiError> {
    let box_server = config.get_string(App::TowerBoxServer);
    let current_version = VERSION.to_string();
    let customer_email = config.get_string(AssitantConfigKey::CustomerEmail);
    let customer_wx = config.get_string(AssitantConfigKey::CustomerWx);
    let asset = PublicAsset {
        app_dir: flow::get_root_dir(),
        box_server,
        current_version,
        customer_email,
        customer_wx,
    };
    Ok(bincode_encode(&asset))
}

//默认浏览器打开
#[tauri::command]
pub fn open_url_by_brower(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    app_name: &str,
    path: &str,
) -> Result<(), ApiError> {
    let url = format!("{}{}", config.get_string(app_name), path);
    webview.opener().open_url(url, None::<&str>).unwrap();
    Ok(())
}
//当前窗口打开
#[tauri::command]
pub fn open_url(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    app_name: &str,
    path: &str,
) -> Result<(), ApiError> {
    let url = format!("{}{}", config.get_string(app_name), path);
    let js = format!("window.location.href={:?};", url);
    webview.eval(js).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key() {
        let keys = vec![Key::Control, Key::Alt, Key::Shift, Key::Tab, Key::Escape];
        for key in keys {
            println!("{}", bincode_encode(key));
        }
    }
}
