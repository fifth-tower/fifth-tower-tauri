use std::time::Duration;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionPolicy {
    Exit(Duration),
    ///延时，下一步位置偏移
    Next(Duration, i32),
    ///延时，重试次数
    Retry(Duration, u32),
}

impl ActionPolicy {
    pub fn default_exit() -> Self {
        Self::Exit(Duration::from_millis(500))
    }
    pub fn default_next() -> Self {
        Self::Next(Duration::from_millis(500), 1)
    }
    pub fn default_retry() -> Self {
        Self::Retry(Duration::from_millis(500), 3)
    }
}

#[derive(Clone, Debug)]
pub enum ActionPolicyType {
    Success,
    Fail,
}
impl ActionPolicyType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Success => "Success".into(),
            Self::Fail => "Fail".into(),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Success => "成功策略".into(),
            Self::Fail => "失败策略".into(),
        }
    }
}
