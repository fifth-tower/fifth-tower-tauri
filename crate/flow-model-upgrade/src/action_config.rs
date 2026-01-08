use tower::common::bincode_encode;

use super::*;

///（action,成功策略，失败策略）
pub type ActionConfig = (Action, ActionPolicy, ActionPolicy);

impl Update for ActionConfig {
    fn update_action(&mut self, index: usize, new: &Action) {
        if let Action::SubFlow(actions, _) = &mut self.0 {
            actions.update_action(index, new);
        }
    }
    fn update_sub_flow(&mut self, index: usize, new: &Actions) {
        if let Action::SubFlow(actions, _) = &mut self.0 {
            actions.update_sub_flow(index, new);
        }
    }
}

impl From<Action> for ActionConfig {
    fn from(value: Action) -> Self {
        ActionConfig::from(&value)
    }
}

impl From<&Action> for ActionConfig {
    fn from(value: &Action) -> Self {
        (
            value.to_owned(),
            ActionPolicy::default_next(),
            ActionPolicy::default_exit(),
        )
    }
}

impl From<&CustomAction> for ActionConfig {
    fn from(value: &CustomAction) -> Self {
        Action::Custom(bincode_encode(value)).into()
    }
}

impl From<CustomAction> for ActionConfig {
    fn from(value: CustomAction) -> Self {
        ActionConfig::from(&value)
    }
}
