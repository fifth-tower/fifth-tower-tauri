use serde::{Deserialize, Serialize};
use tower::common::bincode_encode;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    //ImageArea,template,width,height,rel-x,rel-y,matchMethod,match_value
    Image(Rect, String, u32, u32, i32, i32, MatchMethod, f32),
    //x,y,refer
    Move(i32, i32, String),
    //x,y,refer
    Click(i32, i32, String),
    //x,y,text,refer
    Input(i32, i32, String, String),
    ///x,y,control_1, control_2, key, if len==0 means nothing
    KeyCombi(i32, i32, String, String, String),
    //flow_id
    IncludeFlow(String),
    SubFlow(Actions, u32),
    /// (len,is_vertical,x,y,refer)
    /// length - Number of 15° (click) rotations of the mouse wheel to scroll.
    ///  How many lines will be scrolled depends on the current setting of the operating system.
    /// With Vertical, a positive length will result in scrolling down and negative ones up.
    /// With Horizontal, a positive length will result in scrolling to the right and negative ones to the left
    Scroll(i32, bool, i32, i32, String),
    /// GuaGuaLe(Rect, refer) - 用于模拟刮刮乐操作
    GuaGuaLe(Rect, String),
    Noop(String),
    RecordError(String),
    Custom(String),
}

impl From<CustomAction> for Action {
    fn from(value: CustomAction) -> Self {
        Action::from(&value)
    }
}

impl From<&CustomAction> for Action {
    fn from(value: &CustomAction) -> Self {
        Action::Custom(bincode_encode(value))
    }
}
