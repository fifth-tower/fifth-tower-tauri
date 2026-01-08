use tower::common::bincode_encode;
use tower::common::ApiError;
use tracing::debug;

use crate::common::get_window_info;

///根据app_id获取window信息
#[tauri::command]
pub async fn get_windows(app_ids: Vec<String>) -> Result<String, ApiError> {
    let ret = get_window_info(app_ids);
    debug!("get_windows: {:?}", ret);
    Ok(bincode_encode(ret))
}
