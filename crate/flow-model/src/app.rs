use crate::{Flow, Tag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
    pub app_id: String,
    pub app_name: String,
    pub width: u32,
    pub height: u32,
    pub flows: Vec<Flow>,
    pub virtual_scroll: bool,
    ///光标随机偏移(x,y)
    pub cursor_offset: (i32, i32),
    ///sleep时间随机偏移时长,毫秒数
    pub sleep_offset: i32,
    pub version: String,
    pub zip_name: Option<String>,
    pub zip_description: Option<String>,
    pub tags: Vec<Tag>,
    pub fee: i64,
    pub backend: bool,
}

impl App {
    pub fn new(app_id: &str, app_name: &str, width: u32, height: u32) -> Self {
        Self {
            app_id: app_id.to_string(),
            app_name: app_name.to_string(),
            width,
            height,
            flows: vec![],
            virtual_scroll: true,
            cursor_offset: (10, 10),
            sleep_offset: 50,
            version: "0.0.1".into(),
            zip_name: None,
            zip_description: None,
            tags: vec![],
            fee: 0,
            backend: false,
        }
    }

    pub fn add_flow(&mut self, flow: Flow) {
        self.flows.push(flow);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSetting {
    pub app_id: String,
    pub app_name: String,
    pub width: u32,
    pub height: u32,
    pub virtual_scroll: bool,
    ///光标随机偏移(x,y)
    pub cursor_offset: (i32, i32),
    ///sleep时间随机偏移时长,毫秒数
    pub sleep_offset: i32,

    pub version: String,
    pub zip_name: Option<String>,
    pub zip_description: Option<String>,
    pub tags: Vec<Tag>,
    pub fee: i64,
    pub backend: bool,
}

impl AppSetting {
    pub fn of(app: &App) -> AppSetting {
        let &App {
            ref app_id,
            ref app_name,
            width,
            height,
            virtual_scroll,
            cursor_offset,
            sleep_offset,
            ref version,
            ref zip_name,
            ref zip_description,
            ref tags,
            fee,
            backend,
            ..
        } = app;
        Self {
            app_id: app_id.clone(),
            app_name: app_name.clone(),
            width,
            height,
            virtual_scroll,
            cursor_offset,
            sleep_offset,
            version: version.clone(),
            zip_name: zip_name.clone(),
            zip_description: zip_description.clone(),
            tags: tags.clone(),
            fee,
            backend,
        }
    }

    pub fn to_app(&self) -> App {
        let &Self {
            ref app_id,
            ref app_name,
            width,
            height,
            virtual_scroll,
            cursor_offset,
            sleep_offset,
            ref version,
            ref zip_name,
            ref zip_description,
            ref tags,
            fee,
            backend,
        } = self;

        App {
            app_id: app_id.clone(),
            app_name: app_name.clone(),
            width,
            height,
            virtual_scroll,
            cursor_offset,
            sleep_offset,
            version: version.clone(),
            flows: vec![],
            zip_name: zip_name.clone(),
            zip_description: zip_description.clone(),
            tags: tags.clone(),
            fee,
            backend,
        }
    }
}
