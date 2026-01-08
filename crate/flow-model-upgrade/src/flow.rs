use serde::{Deserialize, Serialize};
use tracing::warn;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flow {
    pub flow_id: String,
    pub flow_name: String,
    pub endpoint: bool,
    pub actions: Vec<ActionConfig>,
}
impl Flow {
    pub fn new(flow_id: &str, flow_name: &str) -> Self {
        Self {
            flow_id: flow_id.to_owned(),
            flow_name: flow_name.to_owned(),
            endpoint: false,
            actions: vec![],
        }
    }
    pub fn of(flow_id: String, flow_name: String, actions: Vec<ActionConfig>) -> Self {
        Self {
            flow_id,
            flow_name,
            endpoint: false,
            actions,
        }
    }
    pub fn flow_name(&self) -> String {
        self.flow_name.clone()
    }
    pub fn flow_id(&self) -> String {
        self.flow_id.clone()
    }
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action.into());
    }
    pub fn add_action_config(&mut self, action_config: ActionConfig) {
        self.actions.push(action_config);
    }
    pub fn update_sub_flow(&mut self, idxs: Vec<usize>, actions: &Actions) {
        let mut indexs = idxs.clone();
        indexs.reverse();
        let mut current_index = indexs.pop();

        let mut children: Vec<Actions> = vec![self.actions.to_owned()];

        while let Some(index) = current_index {
            if let Some(config) = children.last().map(|actions| actions.get(index)).flatten() {
                if let Action::SubFlow(actions, _) = config.0.clone() {
                    if indexs.len() > 0 {
                        children.push(actions);
                    }
                    current_index = indexs.pop();
                } else {
                    warn!("update_sub_flow: 非法路径");
                    break;
                }
            }
        }

        let mut current = actions.to_owned();
        for (index, mut actions) in idxs.iter().zip(children).rev() {
            actions.update_sub_flow(*index, &current);
            current = actions;
        }
        self.actions = current;
    }
}

impl From<&ActionConfig> for Flow {
    fn from(value: &ActionConfig) -> Self {
        let mut flow = Flow::new("".into(), "".into());
        flow.actions = vec![value.to_owned()];
        flow
    }
}

impl From<ActionConfig> for Flow {
    fn from(value: ActionConfig) -> Self {
        Flow::from(&value)
    }
}

impl From<&Action> for Flow {
    fn from(value: &Action) -> Self {
        Flow::from(ActionConfig::from(value))
    }
}

impl From<Action> for Flow {
    fn from(value: Action) -> Self {
        Flow::from(&value)
    }
}
impl From<&CustomAction> for Flow {
    fn from(value: &CustomAction) -> Self {
        Flow::from(ActionConfig::from(value))
    }
}

impl From<CustomAction> for Flow {
    fn from(value: CustomAction) -> Self {
        Flow::from(&value)
    }
}
