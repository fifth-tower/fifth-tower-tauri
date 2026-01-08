use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScrollAttr {
    pub x: i32,
    pub y: i32,
    pub refer: String,
    pub len: i32,
    pub is_vertical: bool,
}

impl TryFrom<&Action> for ScrollAttr {
    type Error = String;

    fn try_from(value: &Action) -> Result<Self, Self::Error> {
        let &Action::Scroll(len, is_vertical, x, y, ref refer) = value else {
            return Err("action非Scroll".to_string());
        };
        Ok(Self {
            x,
            y,
            len,
            is_vertical,
            refer: refer.to_owned(),
        })
    }
}
impl TryFrom<Action> for ScrollAttr {
    type Error = String;

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
