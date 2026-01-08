use flow_model::AppInfo;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExecuteScriptsKey {
    #[serde(rename = "a")]
    pub app_id: String,
    #[serde(rename = "z")]
    pub zip_id: String,
    #[serde(rename = "p")]
    pub pid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallScriptItem {
    pub zip_id: String,
    pub score: i64,
    pub app_info: AppInfo,
}
