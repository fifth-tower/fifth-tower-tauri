use tower::common::bincode_encode;
use tower::common::ApiError;
use tower::common::App;
use tower::common::ConfigGetter;
use tower::config_model::Config;

use crate::command::auth::get_login_user;

#[tauri::command]
pub async fn get_user_apps(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, principal) = get_login_user(&webview, &tower_server).await?;

    let apps: Vec<App> = principal
        .apps
        .iter()
        .filter_map(|app| App::try_from(app.clone()).ok())
        .collect();
    Ok(bincode_encode(apps))
}
