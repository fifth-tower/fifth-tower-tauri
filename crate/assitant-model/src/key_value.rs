use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub name: String,
    pub value: String,
}
