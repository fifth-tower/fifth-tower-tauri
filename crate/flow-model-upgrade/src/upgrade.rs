use ::flow::Dir;
use flow_model::{
    Action as Action2, ActionConfig as ActionConfig2, ActionPolicy as ActionPolicy2,
    Actions as Actions2, App as App2, Flow as Flow2, MatchMethod as ActionMethod2, Rect as Rect2,
};
use fs_extra::{dir, file::write_all};
use tower::common::bincode_encode;

use super::*;

pub fn upgrade_app(app_id: &str) {
    let mut app = store::record::load_app(&Dir::Record, app_id);
    app.app_id = bincode_encode(&app.app_name);
    dir::create_all(Dir::Upgrade.app(&app.app_id), false).unwrap();

    let new_app = upgrade_app_internal(&app);

    println!("{:?}", new_app);
    let content = bincode_encode(new_app);
    write_all(Dir::Upgrade.app_file(&app.app_id), &content).unwrap();
}

pub fn upgrade_flows(app_id: &str) {
    let app = store::record::load_app(&Dir::Record, app_id);
    let app_id = bincode_encode(&app.app_name);

    dir::create_all(Dir::Upgrade.app(&app_id), false).unwrap();
    for flow in &app.flows {
        let app = app.clone();
        let mut new_flow = Flow2::new(&flow.flow_id, &flow.flow_name);
        new_flow.endpoint = flow.endpoint;
        new_flow.actions = upgrade_actions(&flow.actions, &app);

        let content = bincode_encode(new_flow);
        write_all(Dir::Upgrade.flow(&app_id, &flow.flow_id), &content).unwrap();
    }
}

fn upgrade_app_internal(app: &App) -> App2 {
    let new_app = App2::new(&app.app_id, &app.app_name, app.width, app.height);
    new_app
}

fn upgrade_actions(actions: &Actions, app: &App) -> Actions2 {
    let mut new_actions = vec![];
    for ac in actions {
        new_actions.push(upgrade_ac(&ac, app));
    }
    new_actions
}
fn upgrade_ac(ac: &ActionConfig, app: &App) -> ActionConfig2 {
    let (a, sp, fp) = ac;
    (
        upgrade_a(a, app),
        upgrade_policy(sp, app),
        upgrade_policy(fp, app),
    )
}
fn upgrade_a(a: &Action, app: &App) -> Action2 {
    match a {
        &Action::Image(
            match_area,
            ref template,
            width,
            height,
            rel_x,
            rel_y,
            ref action_method,
            match_value,
        ) => Action2::Image(
            Rect2::new(
                match_area.x,
                match_area.y,
                match_area.width,
                match_area.height,
            ),
            template.clone(),
            width,
            height,
            rel_x,
            rel_y,
            ActionMethod2::default(),
            match_value,
        ),
        Action::Move(x, y, refer) => Action2::Move(*x, *y, refer.clone()),
        Action::Click(x, y, refer) => Action2::Click(*x, *y, refer.clone()),
        Action::Input(x, y, tpl, refer) => Action2::Input(*x, *y, tpl.clone(), refer.clone()),
        Action::IncludeFlow(tpl) => Action2::IncludeFlow(tpl.clone()),
        Action::SubFlow(items, times) => Action2::SubFlow(upgrade_actions(items, app), *times),
        Action::Scroll(len, vertical, x, y, refer) => {
            Action2::Scroll(*len, *vertical, *x, *y, refer.clone())
        }
        Action::Noop(str) => Action2::Noop(str.clone()),
        Action::RecordError(str) => Action2::RecordError(str.clone()),
        Action::KeyCombi(x, y, control_1, control_2, key) => {
            Action2::KeyCombi(*x, *y, "ddFHdd".into(), control_2.clone(), key.clone())
        }
        Action::GuaGuaLe(rect, refer) => Action2::GuaGuaLe(
            Rect2::new(rect.x, rect.y, rect.width, rect.height),
            refer.clone(),
        ),
        Action::Custom(str) => Action2::Custom(str.to_owned()),
    }
}
fn upgrade_policy(p: &ActionPolicy, app: &App) -> ActionPolicy2 {
    match p {
        ActionPolicy::Exit(duration) => ActionPolicy2::Exit(*duration),
        ActionPolicy::Next(duration, offset) => ActionPolicy2::Next(*duration, *offset),
        ActionPolicy::Retry(duration, times) => ActionPolicy2::Retry(*duration, *times),
    }
}

#[cfg(test)]
mod tests {
    use std::{env, time::SystemTime};

    use fs_extra::file::read_to_string;
    use tower::common::{bincode_decode, bincode_encode};

    use super::*;

    #[test]
    fn test_upgrade_app() {
        let app_dir = dirs::data_dir()
            .map(|dir| dir.join("assitant"))
            .unwrap()
            .to_string_lossy()
            .to_string();
        ::flow::init_root_dir(&app_dir);

        let app_id = "dd5ddwHtdAOMdhg6dbdkUQNMPuURuy5PUy6Svz7";

        upgrade_app(app_id);
    }
    #[test]
    fn test_upgrade_flows() {
        let app_dir = dirs::data_dir()
            .map(|dir| dir.join("assitant"))
            .unwrap()
            .to_string_lossy()
            .to_string();
        ::flow::init_root_dir(&app_dir);

        let app_id = "dd5ddwHtdAOMdhg6dbdkUQNMPuURuy5PUy6Svz7";

        upgrade_flows(app_id);
    }
    #[test]
    fn test_flows() {
        let app_dir = dirs::data_dir()
            .map(|dir| dir.join("assitant"))
            .unwrap()
            .to_string_lossy()
            .to_string();
        ::flow::init_root_dir(&app_dir);

        let app_id = "dd5ddwHtdAOMdhg6dbdkUQNMPuURuy5PUy6Svz7";
        let flow_id = "flow-1749104339";

        let dir = Dir::Upgrade.flow(app_id, flow_id);
        let content = read_to_string(dir).unwrap();
        let flow: ::flow_model::Flow = bincode_decode(&content);
        println!("{:?}", flow);
    }

    #[test]
    fn test_bincode_encode() {
        let now = SystemTime::now();
        let str = bincode_encode("梦幻西游：时空");
        println!("{}", str);
        println!("{}", bincode_decode::<String>(&str));
        println!("{}", now.elapsed().unwrap().as_millis());
    }
}
