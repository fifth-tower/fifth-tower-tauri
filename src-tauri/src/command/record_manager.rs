use flow::{adjust_image, record, Dir};
use flow_model::{AppSetting, Flow};
use fs_extra::{dir, remove_items};
use tower::common::bincode_decode;
use tower::common_rs::{delete_file, now_secs};

///录制-删除应用
#[tauri::command]
pub fn delete_app(app_id: &str) {
    let dir = Dir::Record.app(app_id);
    dir::create(&dir, true).unwrap();
    remove_items(&vec![dir]).unwrap();
}
///录制-更新应用设置
#[tauri::command]
pub fn update_app_setting(_app_id: String, setting: String) {
    let setting: AppSetting = bincode_decode(&setting);
    record::save_app_setting(&Dir::Record, &setting);
}
///录制-删除流程
#[tauri::command]
pub fn delete_record_flow(app_id: String, flow_id: String) {
    delete_file(Dir::Record.flow(&app_id, &flow_id));

    let image_path = Dir::Record.image(&app_id, Some(&flow_id)).0;
    dir::create(&image_path, true).unwrap();

    remove_items(&vec![image_path]).unwrap();
}
///录制-保存流程
#[tauri::command]
pub fn save_record_flow(app_id: String, indexs: Vec<u32>, flow_str: String) {
    let mut sub_flow: Flow = bincode_decode(&flow_str);
    adjust_image(&mut sub_flow, &app_id, &Dir::Record);
    record::update_sub_flow(&Dir::Record, &&app_id, indexs, &sub_flow);
}
///录制-获取应用列表
#[tauri::command]
pub fn get_record_apps() -> String {
    record::load_apps_for_web(&Dir::Record)
}

///录制-克隆流程
#[tauri::command]
pub fn clone_flow(app_id: String, flow_str: String) {
    let mut flow: Flow = bincode_decode(&flow_str);
    let flow_id = format!("flow-{}", now_secs());
    flow.flow_id = flow_id.clone();
    flow.flow_name = format!("{}_1", flow.flow_name);

    dir::create_all(Dir::Record.image(&app_id, Some(&flow_id)).0, true).unwrap();

    adjust_image(&mut flow, &app_id, &Dir::Record);

    record::save_flow(&Dir::Record, &app_id, &flow);
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

    #[test]
    fn it_works() {
        env::set_var("PROJECT_DIR", "D:/project/assitant");
        let app_id = "test_app";
        let flow_id = "test_flow";
        dir::create_all(Dir::Record.image(&app_id, Some(&flow_id)).0, true).unwrap();
    }
}
