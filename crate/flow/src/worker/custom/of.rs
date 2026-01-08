use crate::{Executable, FlowExecutor};
use flow_model::Action;

pub struct OfWorker;

impl OfWorker {
    pub fn do_work(executor: &mut FlowExecutor, actions: &Vec<Action>) -> bool {
        let mut ret = false;

        executor.start_of();
        for action in actions {
            ret &= action.execute(executor);

            if !ret {
                break;
            }
        }
        executor.end_of();

        ret
    }
}
