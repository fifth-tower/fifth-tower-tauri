use std::{
    fs,
    io::{self, Read},
    path::Path,
};

use flow_model::{App, AppInfo, Flow};
use serde::de::DeserializeOwned;
use tower::common::{bincode_decode, bincode_encode};
use tower::common_rs::{as_zip_password, get_files};
use tracing::debug;
use zip::ZipArchive;

use crate::Dir;

///load app in zip_ids with flows , ignore the zip_id of dir
/// (app_id,script_dir)
pub fn load_apps_with_flows_for_web(dirs: &Vec<(String, Dir)>) -> String {
    let apps: Vec<(String, AppInfo)> = dirs
        .into_iter()
        .filter_map(|(app_id, dir)| {
            let Dir::Script(_, zip_id) = dir else {
                panic!("invalid dir：{}", dir.path())
            };
            load_app(dir, app_id, true)
                .as_ref()
                .map(|f| (zip_id.clone(), f.into()))
        })
        .collect();
    debug!("load_app_infos_for_web: {:?}", apps);
    bincode_encode(apps)
}

pub fn load_app(dir: &Dir, app_id: &str, only_endpoint: bool) -> Option<App> {
    let Dir::Script(uid, _) = dir else {
        panic!("invalid dir：{}", dir.path())
    };
    let path = dir.app(app_id);
    let password = as_zip_password(&uid);

    load_app_by_path(&Path::new(&path), &password, only_endpoint)
}

///load app and flows
/// if only_endpoint is true, only load the endpoint flow
fn load_app_by_path(path: &Path, password: &str, only_endpoint: bool) -> Option<App> {
    let zip_file = fs::File::open(path).unwrap();
    let mut zip_file = ZipArchive::new(zip_file).unwrap();
    zip_file_with_password(&mut zip_file, &Dir::app_file_name(), password).map(|mut app: App| {
        let files: Vec<String> = zip_file.file_names().map(|f| f.into()).collect();
        for file_name in files {
            if !Dir::is_flow_file(&file_name) {
                continue;
            }
            zip_file_with_password(&mut zip_file, &file_name, password).map(|flow: Flow| {
                if only_endpoint && !flow.endpoint {
                    return;
                }
                app.add_flow(flow);
            });
        }
        app.flows.sort_by(|a, b| a.flow_name.cmp(&b.flow_name));
        app
    })
}

pub fn load_flow(dir: &Dir, app_id: &str, flow_id: &str) -> Option<Flow> {
    let Dir::Script(uid, _) = dir else {
        panic!("invalid dir：{}", dir.path())
    };
    let path = dir.app(app_id);

    load_flow_by_path(&Path::new(&path), flow_id, &as_zip_password(&uid))
}

fn load_flow_by_path(path: &Path, flow_id: &str, password: &str) -> Option<Flow> {
    let zip_file = fs::File::open(path).unwrap();
    let mut zip_file = ZipArchive::new(zip_file).unwrap();
    zip_file_with_password(&mut zip_file, &Dir::flow_file_name(flow_id), password)
}

///仅load app_setting, ignore the zip_id of dir
/// //vec<(zip_id,app_info)>
pub fn get_installed_zips(dir: &Dir) -> Vec<(String, AppInfo)> {
    let Dir::Script(uid, _) = dir else {
        panic!("invalid dir")
    };
    let dirs = get_files(dir.path(), 2);
    debug!("dirs:{:?}", dirs);
    let ret = dirs
        .iter()
        .filter_map(|d| {
            let path = Path::new(d);
            is_script(path)
                .then(|| load_app_info_with_entry(path, &as_zip_password(&uid)))
                .flatten()
        })
        .collect();
    debug!("get_installed_zips:{:?}", ret);
    ret
}

///是否秘籍
fn is_script(path: &Path) -> bool {
    path.file_name()
        .map(|f| Dir::is_script_file(&f.to_string_lossy()))
        .unwrap_or(false)
}

fn load_app_info_with_entry(path: &Path, password: &str) -> Option<(String, AppInfo)> {
    let zip_id = path.file_name().map(|f| f.to_string_lossy()).unwrap();
    let zip_id = Dir::zip_id(&zip_id).unwrap();
    let zip_file = fs::File::open(path).unwrap();
    let mut zip_file = ZipArchive::new(zip_file).unwrap();
    zip_file_with_password(&mut zip_file, &Dir::app_file_name(), password)
        .map(|app: App| (zip_id.to_string(), AppInfo::from(&app)))
}

fn zip_file_with_password<T, R>(
    zip_file: &mut ZipArchive<R>,
    file_name: &str,
    password: &str,
) -> Option<T>
where
    T: DeserializeOwned,
    R: io::Read + io::Seek,
{
    zip_file
        .by_name_decrypt(file_name, password.as_bytes())
        .map(|mut f| {
            let mut content = "".to_string();
            f.read_to_string(&mut content).unwrap();

            bincode_decode(&content)
        })
        .ok()
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;

    #[test]
    fn it_works2() {
        env::set_var("PROJECT_DIR", "D:/project/assitant");
    }
}
