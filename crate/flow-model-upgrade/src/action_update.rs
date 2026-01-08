use super::*;

///self 为subflow
pub trait Update {
    fn update_action(&mut self, index: usize, new: &Action);
    fn update_sub_flow(&mut self, index: usize, new: &Actions);
}
