use super::*;
use crate::record::{load_app_setting, load_flow, save_app_setting, save_flow};
use flow_model::*;
use fs_extra::{dir, remove_items};
use tower::{common::str_to_filename, common_rs::now_secs};
use tracing::{debug, warn};

#[derive(Clone, Debug)]
pub struct Recording {
    pid: Option<u32>,
    flow: Option<Flow>,
    app_id: Option<String>,
    actions: Vec<Actions>,
    master_actions: Actions,
    app_setting: Option<AppSetting>,
}
impl Recording {
    pub fn new() -> Self {
        Self {
            pid: None,
            flow: None,
            app_id: None,
            actions: vec![],
            master_actions: vec![],
            app_setting: None,
        }
    }
    ///重新录制
    fn start_with_flow(&mut self, pid: u32, app_id: &str, flow_id: &str) {
        let flow = load_flow(&Dir::Record, app_id, flow_id);

        self.pid = Some(pid);
        self.app_id = Some(app_id.to_string());
        self.flow = Some(flow.clone());
        self.actions = vec![flow.actions];
        self.master_actions = vec![];
        self.app_setting = load_app_setting(&Dir::Record, app_id);

        self.init_dir();
    }

    pub fn last_action(&self) -> Option<ActionConfig> {
        self.actions.last().and_then(|f| f.last()).cloned()
    }

    pub fn start(&mut self, pid: u32, app_id: Option<&str>, flow_id: Option<&str>) {
        if flow_id.is_some() {
            self.start_with_flow(pid, app_id.unwrap(), flow_id.unwrap());
            return;
        }
        let window = get_window(pid);
        if window.is_none() {
            warn!("未找到应用,pid={}", pid);
            return;
        }
        let window = window.unwrap();
        let app_name = window.app_name().unwrap();
        let flow_id = format!("flow-{}", now_secs());
        let app_id = str_to_filename(&app_name);
        let width = window.width().unwrap();
        let height = window.height().unwrap();

        debug!("recording:pid:{pid},app_id:{app_id:?},flow_id:{flow_id:?},dim:({width},{height})");

        self.pid = Some(pid);
        self.flow = Some(Flow::new(&flow_id, &flow_id));
        self.app_id = Some(app_id.clone());
        self.actions.push(vec![]);

        self.init_dir();
        self.app_setting = load_app_setting(&Dir::Record, &app_id).or_else(|| {
            let app = App::new(&app_id, &app_name, width, height);
            let app = AppSetting::of(&app);
            save_app_setting(&Dir::Record, &app);
            Some(app)
        });
    }
    fn init_dir(&self) {
        let app_id = self.app_id();
        let flow_id = self.flow_id();

        dir::create_all(Dir::Record.image(&app_id, Some(&flow_id)).0, false).unwrap();
    }
    fn remove_dir(&self) {
        if self.app_id.is_none() {
            return;
        }
        let app_id = self.app_id();
        let flow_id = self.flow_id();

        if !record::exists_flow(&Dir::Record, &app_id, &flow_id) {
            remove_items(&vec![Dir::Record.image(&app_id, Some(&flow_id)).0]).unwrap();
        }
    }
    pub fn cancel(&mut self) {
        self.remove_dir();
        self.reset();
    }
    pub fn add_action(&mut self, action_type: ActionCommand) -> ActionConfig {
        debug!("add_action start:{:?}", action_type);

        let Rect {
            x: win_x, y: win_y, ..
        } = get_window_rect_by_pid(self.pid.unwrap());
        let action = match action_type {
            ActionCommand::Image(
                rect,
                template,
                width,
                height,
                rel_x,
                rel_y,
                method,
                match_value,
            ) => Action::Image(
                rect.to_relative(win_x, win_y),
                template,
                width,
                height,
                rel_x,
                rel_y,
                method,
                match_value,
            ),
            ActionCommand::Move(x, y, refer) => Action::Move(x - win_x, y - win_y, refer),
            ActionCommand::Click(x, y, refer) => Action::Click(x - win_x, y - win_y, refer),
            ActionCommand::GuaGuaLe(rect, refer) => {
                Action::GuaGuaLe(rect.to_relative(win_x, win_y), refer)
            }
            ActionCommand::Scroll(len, is_vertical, x, y, refer) => {
                Action::Scroll(len, is_vertical, x - win_x, y - win_y, refer)
            }
            ActionCommand::Input(x, y, refer) => Action::Input(x, y, "text".into(), refer),
            ActionCommand::KeyCombi(x, y) => {
                Action::KeyCombi(x - win_x, y - win_y, "".into(), "".into(), "".into())
            }
            ActionCommand::IncludeFlow => Action::IncludeFlow("include-flow-id".to_string()),
            ActionCommand::SubFlowStart => {
                self.actions.push(vec![]);
                Action::Noop("录制子流程开始".into())
            }
            ActionCommand::SubFlowEnd => {
                let last = self.actions.pop();
                match last {
                    Some(last) => {
                        let action = Action::SubFlow(last, 1);

                        self.actions.last_mut().map(|a| a.push(action.into()));

                        Action::Noop("录制子流程结束".into())
                    }
                    None => Action::RecordError("子流程录制异常".into()),
                }
            }
        };
        if !matches!(action, Action::Noop(_)) {
            let last = self.actions.last_mut();
            match last {
                Some(last) => {
                    last.push(ActionConfig::from(&action));
                }
                None => self
                    .actions
                    .push(vec![Action::RecordError("录制动作时异常".into()).into()]),
            }
        }
        debug!("add_action end:{:?}", action);
        action.into()
    }

    fn flush(&mut self) -> Result<(), Action> {
        while self.actions.len() > 1 {
            self.add_action(ActionCommand::SubFlowEnd);
        }
        self.actions
            .last_mut()
            .map(|a| self.master_actions.append(a));

        self.actions.clear();
        Ok(())
    }
    fn write(&mut self) {
        self.flow
            .as_mut()
            .map(|f| f.actions = self.master_actions.clone());
        save_flow(&Dir::Record, &self.app_id(), self.flow.as_ref().unwrap());

        self.master_actions.clear();
    }

    pub fn end(&mut self) {
        self.flush().unwrap();
        self.write();
        self.reset();
    }

    pub fn reset(&mut self) {
        self.pid = None;
        self.app_id = None;
        self.flow = None;
        self.actions = vec![];
        self.master_actions = vec![]
    }
    pub fn inited(&self) -> bool {
        self.app_id.is_some()
    }

    pub fn flow_id(&self) -> String {
        self.flow.as_ref().map(|f| f.flow_id.clone()).unwrap()
    }
    pub fn app_id(&self) -> String {
        self.app_id.clone().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{env, time::SystemTime};

    use super::*;

    #[test]
    fn it_works() {
        env::set_var("PROJECT_DIR", "D:/project/assitant");
        let now = SystemTime::now();
        let mut rec = Recording::new();
        rec.start(7624, None, None);
        println!("{:?}", now.elapsed());
    }
}
