use flow_model::*;

use super::*;

pub trait Executable {
    fn execute(&self, executor: &mut FlowExecutor) -> bool;
    fn execute_repeat(&self, executor: &mut FlowExecutor, times: u32) -> bool {
        let mut ret = true;
        let mut i = 0;
        while i < times {
            if times > 1 {
                executor.write_log(LogContent::Loop(i + 1));
            }
            ret = self.execute(executor);
            if times > 1 {
                executor.write_log(LogContent::LoopResult(i + 1, ret));
            }

            if !ret {
                break;
            }
            i += 1;
        }
        ret
    }
}
impl Executable for Flow {
    fn execute(&self, executor: &mut FlowExecutor) -> bool {
        executor.push_flow_id(self.flow_id.clone());
        executor.add_action_level();
        executor.write_log(LogContent::Flow(
            self.flow_id.clone(),
            self.flow_name.clone(),
        ));

        let ret = self.actions.execute(executor);

        executor.write_log(LogContent::FlowResult(
            self.flow_id.clone(),
            self.flow_name.clone(),
            ret,
        ));
        executor.sub_action_level();
        executor.pop_flow_id();
        ret
    }
}
impl Executable for Actions {
    //ret 期望继续往下执行时，返回true.
    fn execute(&self, executor: &mut FlowExecutor) -> bool {
        let mut ret = true;
        executor.add_action_level();

        let mut index = 0;
        let mut success_exit = false;
        while let Some(action_config) = self.get(index) {
            let mut result = action_config.execute(executor);

            if !success_exit && matches!(action_config.1, ActionPolicy::Exit(_)) {
                success_exit = true;
            }
            //应用策略
            if result {
                match &action_config.1 {
                    ActionPolicy::Exit(duration) => {
                        executor.sync_sleep_with_offset(duration);
                        //满足条件，退出
                        executor.sub_action_level();
                        return true;
                    }
                    ActionPolicy::Next(duration, offset) => {
                        index = (index as i32 + offset) as usize;
                        executor.sync_sleep_with_offset(duration);
                    }
                    ActionPolicy::Retry(_, _) => todo!(),
                }
                ret = result;
            } else {
                match &action_config.2 {
                    ActionPolicy::Exit(duration) => {
                        ret = result;
                        executor.sync_sleep_with_offset(duration);
                        break;
                    }
                    ActionPolicy::Next(duration, offset) => {
                        result = true;
                        index = (index as i32 + offset) as usize;
                        executor.sync_sleep_with_offset(duration);
                    }
                    ActionPolicy::Retry(duration, times) => {
                        //失败重试
                        let mut exec_times = 0;

                        while exec_times.lt(times) {
                            executor.write_log(LogContent::Retry(exec_times + 1));

                            executor.sync_sleep_with_offset(duration);
                            result = action_config.execute(executor);

                            executor.write_log(LogContent::RetryResult(exec_times + 1, result));

                            //如果成功，应用成功策略
                            if result {
                                match &action_config.1 {
                                    ActionPolicy::Exit(duration) => {
                                        executor.sync_sleep_with_offset(duration);
                                        //满足条件，退出
                                        executor.sub_action_level();
                                        return true;
                                    }
                                    ActionPolicy::Next(duration, offset) => {
                                        index = (index as i32 + offset) as usize;
                                        executor.sync_sleep_with_offset(duration);
                                    }
                                    ActionPolicy::Retry(_, _) => todo!(),
                                }
                                break;
                            }
                            exec_times += 1;
                        }
                    }
                }
                ret = result;
                //失败，退出当前(子)流程
                if !ret {
                    break;
                }
            };
        }
        executor.sub_action_level();
        if success_exit {
            false
        } else {
            ret
        }
    }
}
impl Executable for ActionConfig {
    fn execute(&self, executor: &mut FlowExecutor) -> bool {
        if executor.is_stop() {
            executor.write_log(LogContent::Stop);
            panic!("外部终止，停止运行");
        }

        executor.write_log(LogContent::Action(self.clone()));

        if executor.is_minimized() {
            executor.write_log(LogContent::Minimized);
            return false;
        }

        let ret = self.0.execute(executor);
        if let Action::Image(..) = self.0 {
            ret
        } else {
            executor.write_log(LogContent::ActionResult(self.clone(), ret));
            ret
        }
    }
}
impl Executable for Action {
    fn execute(&self, executor: &mut FlowExecutor) -> bool {
        match self {
            Action::Image(..) => {
                let image_attr = self.try_into().unwrap();
                let (matched, (match_value, match_location)) =
                    ImageClickWorker::do_work(executor, &image_attr);

                executor.write_log(LogContent::Matched(matched, match_value, match_location));

                matched
            }
            Action::Move(x, y, _) => MouseMoveWorker::do_work(executor, (*x, *y)),
            Action::Click(x, y, _) => MouseClickWorker::do_work(executor, (*x, *y)),
            Action::Scroll(..) => MouseScrollWorker::do_work(executor, &self.try_into().unwrap()),
            Action::GuaGuaLe(rect, _) => GuagualeWorker::do_work(executor, rect),
            Action::Input(x, y, str, _) => InputTextWorker::do_work(executor, (*x, *y), str),
            Action::KeyCombi(..) => {
                let key_combi_attr = self.try_into().unwrap();
                KeyCombiWorker::do_work(executor, &key_combi_attr)
            }
            Action::Custom(action) => CustomActionWorker::do_work(executor, action.into()),
            Action::IncludeFlow(flow_id) => executor.flow(flow_id).execute(executor),
            Action::SubFlow(actions, times) => actions.execute_repeat(executor, *times),
            Action::Noop(_) => true,
            Action::RecordError(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{os::windows::thread, sync::Arc, time::Duration};

    use crate::tests::init;
    use std::thread::sleep;

    use super::*;

    #[test]
    fn test_action_exec() {
        init();

        let app_id = "FQAAAAAAAADmoqblubvopbmuLjvvJrml7bnqbo=";
        let pid = 5060;

        let app = record::load_app(&Dir::Record, app_id);
        let resource = Arc::new(FLowResource::new());
        let mut executor = FlowExecutor::new(pid, app, resource, Dir::Record, Box::new(|_| {}));

        let action = Action::Scroll(6, true, 810, 513, "".into());
        action.execute(&mut executor);
    }

    #[test]
    fn test_action_exec_async() {
        init();

        tauri::async_runtime::spawn_blocking(move || {
            let app_id = "FQAAAAAAAADmoqblubvopbmuLjvvJrml7bnqbo=";
            let pid = 5060;

            let app = record::load_app(&Dir::Record, app_id);
            let resource = Arc::new(FLowResource::new());
            let mut executor = FlowExecutor::new(pid, app, resource, Dir::Record, Box::new(|_| {}));

            let action = Action::Scroll(6, true, 810, 513, "".into());

            action.execute(&mut executor);
        });
        sleep(Duration::from_secs(10));
    }
}
