//! persient use for record

use flow_model::*;
use fs_extra::file::{read_to_string, write_all};
use tower::common::{bincode_decode, bincode_encode};
use tower::common_rs::{get_dirs, get_file_name, get_files};

use crate::Dir;

///获取当前流程总数
pub fn get_flow_num(dir: &Dir) -> usize {
    let apps = load_apps(dir);
    apps.iter().fold(0, |total, app| total + app.flows.len())
}
///record, load app include flows
pub fn load_apps_for_web(dir: &Dir) -> String {
    let apps = load_apps(dir);
    // debug!("load_apps_for_web:{:?}", apps);
    bincode_encode(apps)
}

pub(crate) fn load_apps(dir: &Dir) -> Vec<App> {
    get_dirs(dir.path(), 1, false)
        .iter()
        .map(|path| load_app(dir, &get_file_name(path)))
        .collect()
}

pub fn load_app_setting(dir: &Dir, app_id: &str) -> Option<AppSetting> {
    read_to_string(dir.app_file(app_id))
        .ok()
        .as_ref()
        .map(|content| {
            let app = bincode_decode(content);
            AppSetting::of(&app)
        })
}

pub fn load_app(dir: &Dir, app_id: &str) -> App {
    let content = read_to_string(dir.app_file(app_id)).unwrap();
    let mut app: App = bincode_decode(&content);
    get_files(dir.app(app_id), 1).iter().for_each(|file| {
        let file_name = get_file_name(file);
        if Dir::is_app_file(&file_name) {
            return;
        }
        if let Some(flow_id) = Dir::flow_id(&file_name) {
            app.add_flow(load_flow(dir, app_id, flow_id));
        }
    });
    app.flows.sort_by(|a, b| a.flow_name.cmp(&b.flow_name));
    app
}

pub(crate) fn exists_flow(dir: &Dir, app_id: &str, flow_id: &str) -> bool {
    let dir = dir.flow(app_id, flow_id);
    read_to_string(dir).is_ok()
}

pub(crate) fn load_flow(dir: &Dir, app_id: &str, flow_id: &str) -> Flow {
    let dir = dir.flow(app_id, flow_id);
    let content = read_to_string(dir).unwrap();
    bincode_decode(&content)
}

pub fn save_app_setting(dir: &Dir, app_setting: &AppSetting) {
    let app = app_setting.to_app();
    let app_path = dir.app_file(&app.app_id);
    let content = bincode_encode(app);

    write_all(app_path, &content).unwrap();
}

///上传秘籍时，同步zip信息
pub fn update_app_with_zip<F>(dir: &Dir, app_id: &str, setter: F) -> Result<(), String>
where
    F: Fn(&mut AppSetting) -> Result<(), String>,
{
    let mut app = load_app_setting(dir, app_id);
    app.as_mut().map_or(Err("未找到应用".into()), |app| {
        let ret = setter(app);
        ret.and_then(|_| {
            save_app_setting(dir, app);
            Ok(())
        })
    })
}

pub fn save_flow(dir: &Dir, app_id: &str, flow: &Flow) {
    let flow_id = flow.flow_id();
    let content = bincode_encode(flow);

    write_all(dir.flow(app_id, &flow_id), &content).unwrap();
}

pub fn update_sub_flow(dir: &Dir, app_id: &str, indexs: Vec<u32>, sub_flow: &Flow) {
    if indexs.len() == 0 {
        save_flow(dir, app_id, sub_flow);
        return;
    }
    let flow_id = sub_flow.flow_id();
    let mut flow = load_flow(dir, app_id, &flow_id);
    flow.flow_name = sub_flow.flow_name();
    flow.endpoint = sub_flow.endpoint;

    let indexs = indexs.iter().map(|f| *f as usize).collect();
    flow.update_sub_flow(indexs, &sub_flow.actions);

    save_flow(dir, app_id, &flow);
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn it_works() {
        // let content = "ddxddCdldWBsdrI_d9de3DxxyDW4DXgkDD1_iut5vrstFtXAjFdvtdwtCHstHP__674FjlkddY8ddyPdbOkPQoMRUJd7UdiPjdM8BdSLd5ZlGJddCddddddfdddddddd5ddd5dddedddCdddcs8dYdmddOdcMfddddddddddddddddddddddddd";
        // let app: App = bincode_decode(content);
        // println!("{:?}", app);
        let app = App::new("dd", "~~~~~~~~~~~~~~~~~~~~~~~~~sssssdd", 1, 1);
        println!("{:?}", app);
        let content = bincode_encode(app);
        println!("{:?}", content);
        println!("{:?}", bincode_decode::<App>(&content));
    }
}
