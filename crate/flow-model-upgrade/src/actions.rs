use super::*;

pub type Actions = Vec<ActionConfig>;

impl Update for Actions {
    fn update_action(&mut self, index: usize, new: &Action) {
        if let Some(action) = self.get_mut(index) {
            action.0 = new.to_owned();
        }
    }
    fn update_sub_flow(&mut self, index: usize, new: &Actions) {
        if let Some(action) = self.get_mut(index) {
            if let Action::SubFlow(_, t) = &mut action.0 {
                action.0 = Action::SubFlow(new.to_owned(), *t);
            }
        }
    }
}
