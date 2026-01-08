use flow_model::CustomAction;

use super::*;
use crate::FlowExecutor;

pub struct CustomActionWorker;
impl CustomActionWorker {
    pub fn do_work(executor: &FlowExecutor, action: CustomAction) -> bool {
        match action {
            CustomAction::GoToSee(x, y) => GoToSeeWorker::do_work(executor, (x, y)),
            CustomAction::Resize => ResizeWindowWorker::do_work(executor),
        }
    }
}
