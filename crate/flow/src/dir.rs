use std::sync::OnceLock;

static DIR_ROOT: OnceLock<String> = OnceLock::new();
const APP_FILE_NAME: &str = "app.setting";
const FLOW_FILE_SUFFIX: &str = ".flow";
const _APP_FILE_SUFFIX: &str = ".setting";
const SCRIPT_FILE_SUFFIX: &str = ".tower";

pub fn init_root_dir(app_dir: &str) {
    DIR_ROOT.get_or_init(|| app_dir.to_string());
}

pub fn get_root_dir() -> String {
    format!("{}/resource", DIR_ROOT.get().cloned().unwrap())
}

#[derive(Clone, Debug)]
pub enum Dir {
    Record,
    ///user_id, zip_id
    Script(String, String),
    Tmp,
    Upgrade,
}

impl Dir {
    pub fn path(&self) -> String {
        let dir = get_root_dir();
        format!("{}{}", dir, self.prefix())
    }
    /// when record return app dir
    /// when script return zip file
    pub fn app(&self, app_id: &str) -> String {
        let Self::Script(_, zip_id) = self else {
            return format!("{}/{}", self.path(), app_id);
        };

        format!(
            "{}/{}/{}{}",
            self.path(),
            app_id,
            zip_id,
            SCRIPT_FILE_SUFFIX
        )
    }
    pub fn app_file(&self, app_id: &str) -> String {
        format!("{}/{}", self.app(app_id), APP_FILE_NAME)
    }
    pub fn image(&self, app_id: &str, flow_id: Option<&str>) -> (String, String) {
        let Self::Script(_, zip_id) = self else {
            return flow_id.map_or(
                (
                    format!("{}/{}", self.path(), app_id),
                    format!("{}/{}", self.prefix(), app_id),
                ),
                |flow_id| {
                    (
                        format!("{}/{}/{}", self.path(), app_id, flow_id),
                        format!("{}/{}/{}", self.prefix(), app_id, flow_id),
                    )
                },
            );
        };
        flow_id.map_or(
            (
                format!("{}/{}/{}", self.path(), app_id, zip_id),
                format!("{}/{}/{}", self.prefix(), app_id, zip_id),
            ),
            |flow_id| {
                (
                    format!("{}/{}/{}/{}", self.path(), app_id, zip_id, flow_id),
                    format!("{}/{}/{}/{}", self.prefix(), app_id, zip_id, flow_id),
                )
            },
        )
    }
    ///flow file
    pub fn flow(&self, app_id: &str, flow_id: &str) -> String {
        format!("{}/{}{}", self.app(app_id), flow_id, FLOW_FILE_SUFFIX)
    }
    pub fn prefix(&self) -> String {
        match self {
            Self::Record => "/record".into(),
            Self::Script(uid, _) => format!("/plugin/{}", uid),
            Self::Tmp => "/tmp".into(),
            Self::Upgrade => "/upgrade".into(),
        }
    }
    ///是否app.setting文件
    pub fn is_app_file(file_name: &str) -> bool {
        APP_FILE_NAME.eq(file_name)
    }
    ///是否flow配置文件
    pub fn is_flow_file(file_name: &str) -> bool {
        file_name.ends_with(FLOW_FILE_SUFFIX)
    }
    ///是否秘籍打包文件 .tower
    pub fn is_script_file(file_name: &str) -> bool {
        file_name.ends_with(SCRIPT_FILE_SUFFIX)
    }
    pub fn flow_id(file_name: &str) -> Option<&str> {
        file_name.strip_suffix(FLOW_FILE_SUFFIX)
    }
    pub fn zip_id(file_name: &str) -> Option<&str> {
        file_name.strip_suffix(SCRIPT_FILE_SUFFIX)
    }
    pub fn flow_file_name(flow_id: &str) -> String {
        format!("{}{}", flow_id, FLOW_FILE_SUFFIX)
    }
    pub fn app_file_name() -> String {
        APP_FILE_NAME.to_string()
    }
}
