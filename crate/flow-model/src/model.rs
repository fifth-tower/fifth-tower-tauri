use crate::{App, AppSetting, Flow};

///web 参数结构

///window进程信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowInfo {
    pub pid: u32,
    pub app_id: String,
    pub app_name: String,
    //path to the snapshot image
    pub snapshot: String,
}

///App信息--我的模块
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppInfo {
    pub app_id: String,
    pub app_name: String,
    pub version: String,
    pub zip_name: Option<String>,
    pub zip_description: Option<String>,
    pub flows: Vec<FlowInfo>,
    pub tags: Vec<Tag>,
}

impl From<&App> for AppInfo {
    fn from(app: &App) -> Self {
        Self {
            app_id: app.app_id.clone(),
            app_name: app.app_name.clone(),
            version: app.version.clone(),
            zip_name: app.zip_name.clone(),
            zip_description: app.zip_description.clone(),
            flows: app.flows.iter().map(|f| f.into()).collect(),
            tags: app.tags.clone(),
        }
    }
}

impl From<&AppSetting> for AppInfo {
    fn from(app: &AppSetting) -> Self {
        Self {
            app_id: app.app_id.clone(),
            app_name: app.app_name.clone(),
            version: app.version.clone(),
            zip_name: app.zip_name.clone(),
            zip_description: app.zip_description.clone(),
            flows: vec![],
            tags: app.tags.clone(),
        }
    }
}
///Flow信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlowInfo {
    pub flow_id: String,
    pub flow_name: String,
}

impl From<&Flow> for FlowInfo {
    fn from(flow: &Flow) -> Self {
        Self {
            flow_id: flow.flow_id.clone(),
            flow_name: flow.flow_name.clone(),
        }
    }
}

///Flow运行状态
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum RunStatus {
    Init,
    Running,
    ///when execute success is true, otherwise false
    Stopped(bool),
    Aborted,
}

//脚本标签
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub typ: ZipTagType,
}
impl Tag {
    pub fn new(id: &str, label: &str, typ: &ZipTagType) -> Self {
        Self {
            id: id.to_owned(),
            label: label.to_owned(),
            typ: typ.to_owned(),
        }
    }
    pub fn class(&self) -> String {
        self.typ.class()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ZipTagType {
    App,
    Version,
    User,
    Org,
}

impl ZipTagType {
    pub fn class(&self) -> String {
        match self {
            Self::App => "warning".into(),
            Self::Version => "info".into(),
            Self::User => "neutral".into(),
            Self::Org => "secondary".into(),
        }
    }
}
