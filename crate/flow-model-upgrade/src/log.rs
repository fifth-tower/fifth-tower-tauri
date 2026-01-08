use serde::{Deserialize, Serialize};

use crate::{Action, ActionConfig};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMessage {
    pub action_level: u32,
    pub content: LogContent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogContent {
    //flow_id,flow_name
    Flow(String, String),
    //flow_id,flow_name,result
    FlowResult(String, String, bool),
    Action(ActionConfig),
    //actionconfig,result
    ActionResult(ActionConfig, bool),
    //第N次重试
    Retry(u32),
    RetryResult(u32, bool),
    //第N次循环
    Loop(u32),
    LoopResult(u32, bool),
    ///图像匹配结果：（is_matched, match_value, match_location）
    Matched(bool, f32, (u32, u32)),
    //手动停止
    Stop,
    Minimized,
}
impl LogMessage {
    //展示时是否需要替换，若为流程，子流程返回false
    pub fn need_replace(&self) -> bool {
        match &self.content {
            LogContent::ActionResult(ac, _) => {
                !matches!(ac.0, Action::IncludeFlow(_) | Action::SubFlow(..))
            }
            _ => false,
        }
    }
}
