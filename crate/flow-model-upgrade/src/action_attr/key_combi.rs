use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyCombiAtrr {
    pub x: i32,
    pub y: i32,
    pub control_1: String,
    pub control_2: String,
    pub key: String,
}

impl TryFrom<&Action> for KeyCombiAtrr {
    type Error = String;

    fn try_from(value: &Action) -> Result<Self, Self::Error> {
        let &Action::KeyCombi(x, y, ref control_1, ref control_2, ref key) = value else {
            return Err("action非KeyCombi".to_string());
        };
        Ok(Self {
            x,
            y,
            control_1: control_1.clone(),
            control_2: control_2.clone(),
            key: key.clone(),
        })
    }
}
impl TryFrom<Action> for KeyCombiAtrr {
    type Error = String;

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}
