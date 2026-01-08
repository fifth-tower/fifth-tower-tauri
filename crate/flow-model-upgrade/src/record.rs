use serde::{Deserialize, Serialize};

///录制类型
///
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RecordType {
    ///录流程
    Flow,
    ///录动作
    Action(usize),
}
