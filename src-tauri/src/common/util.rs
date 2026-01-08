use std::path::PathBuf;

use enigo::{Enigo, Mouse, Settings};
use flow::Dir;
use flow_model::WindowInfo;
use fs_extra::dir;
use tauri::{Manager, Runtime};
use tower::{common::str_to_filename, common_rs::now_secs};
use xcap::Window;

pub fn get_mouse_postion() -> (i32, i32) {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.location().unwrap()
}

pub fn get_window_info(app_ids: Vec<String>) -> Vec<WindowInfo> {
    dir::create_all(Dir::Tmp.path(), true).unwrap();

    let mut pids: Vec<u32> = vec![];
    Window::all()
        .unwrap()
        .iter()
        .filter_map(|w| {
            if w.is_minimized().unwrap() {
                return None;
            }
            let w_pid = w.pid().unwrap();
            let w_app_name = w.app_name().unwrap();
            let snapshot = w.capture_image().unwrap();
            let w_app_id = str_to_filename(&w_app_name);

            if pids.contains(&w_pid) {
                return None;
            }
            if app_ids.len() > 0 && !app_ids.contains(&w_app_id) {
                return None;
            }
            let snapshot_path = Dir::Tmp.image(&format!("{}-{}.png", w_pid, now_secs()), None);
            snapshot.save(snapshot_path.0).unwrap();

            pids.push(w_pid);
            Some(WindowInfo {
                pid: w_pid,
                app_id: w_app_id.clone(),
                app_name: w_app_name.clone(),
                snapshot: snapshot_path.1,
            })
        })
        .collect()
}

pub fn data_dir<T, R>(app: &T) -> PathBuf
where
    T: Manager<R>,
    R: Runtime,
{
    app.path().app_data_dir().unwrap()
}
