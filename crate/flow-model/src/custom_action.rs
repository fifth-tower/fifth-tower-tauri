use serde::{Deserialize, Serialize};
use tower::common::bincode_decode;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CustomAction {
    //x,y
    GoToSee(i32, i32),
    Resize,
}

impl From<&String> for CustomAction {
    fn from(value: &String) -> Self {
        bincode_decode(value)
    }
}
impl ToString for CustomAction {
    fn to_string(&self) -> String {
        match self {
            Self::GoToSee(..) => format!("移动鼠标位置"),
            Self::Resize => format!("调整窗口大小"),
        }
    }
}
