use std::path::Path;

use crate::command::auth::get_login_user;
use crate::common::data_dir;
use crate::common::get_store_name_by_type;
use fs_extra::dir;
use fs_extra::file::write_all;
use fs_extra::remove_items;
use tower::common::bincode_encode;
use tower::common::random_id;
use tower::common::ApiResult;
use tower::common::{App, ConfigGetter};
use tower::common_rs::read_bin_file;
use tower::common_rs::read_file_to_str;
use tower::config_model::Config;
use tower::jwt::Principal;
use tower::tauri_model::StoreFile;
use tower::tauri_model::StoreRoot;
use tower::tauri_model::StoreRootData;
use tower::tauri_model::STORE_ROOT_ID;
use tower::{
    common::{bincode_decode, ApiError},
    tauri_model::StoreType,
};
use tracing::debug;

#[tauri::command]
pub async fn load_stores(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, Principal { user_id, .. }) = get_login_user(&webview, &tower_server).await?;
    let root = get_root_file(&webview, store_type, &user_id)?;
    let mut data = StoreRootData::new(&user_id);
    root.files.iter().for_each(|file_id| {
        let file_path = get_file_path(&webview, store_type, &user_id, file_id);
        if let Ok(file) = read_bin_file::<StoreFile>(&file_path) {
            if user_id.eq(&file.user_id) {
                data.add_file(file);
            }
        }
    });
    Ok(bincode_encode(data))
}

#[tauri::command]
pub async fn set_store_extra_info(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
    extra_info: &str,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, Principal { user_id, .. }) = get_login_user(&webview, &tower_server).await?;

    let mut root = get_root_file(&webview, store_type, &user_id)?;
    root.extra_info = bincode_decode(extra_info);

    let file_path = get_file_path(&webview, store_type, &user_id, STORE_ROOT_ID);
    write_all(file_path, &bincode_encode(root)).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn get_store_extra_info(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, Principal { user_id, .. }) = get_login_user(&webview, &tower_server).await?;

    let root = get_root_file(&webview, store_type, &user_id)?;

    Ok(bincode_encode(root.extra_info))
}

#[tauri::command]
pub async fn create_store(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
    label: &str,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, Principal { user_id, .. }) = get_login_user(&webview, &tower_server).await?;
    //save note
    let file_id = random_id(6);
    let file = StoreFile::new(&file_id, label, &user_id);
    let file_path = get_file_path(&webview, store_type, &user_id, &file_id);
    write_all(file_path, &bincode_encode(file)).unwrap();

    //add to root
    let mut root = get_root_file(&webview, store_type, &user_id)?;
    root.add_file(&file_id);
    let file_path = get_file_path(&webview, store_type, &user_id, STORE_ROOT_ID);
    write_all(file_path, &bincode_encode(root)).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn load_store(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
    file_id: &str,
) -> Result<String, ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, Principal { user_id, .. }) = get_login_user(&webview, &tower_server).await?;
    //delete note
    let file_path = get_file_path(&webview, store_type, &user_id, &file_id);

    Ok(read_file_to_str(&file_path)?)
}

#[tauri::command]
pub async fn delete_store(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
    file_id: &str,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, Principal { user_id, .. }) = get_login_user(&webview, &tower_server).await?;
    //delete note
    let file_path = get_file_path(&webview, store_type, &user_id, &file_id);
    remove_items(&[file_path]).unwrap();

    //remove from root
    let mut root = get_root_file(&webview, store_type, &user_id)?;
    root.remove_file(&file_id);
    let file_path = get_file_path(&webview, store_type, &user_id, STORE_ROOT_ID);
    write_all(file_path, &bincode_encode(root)).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn save_store(
    webview: tauri::WebviewWindow,
    config: tauri::State<'_, Config>,
    store_type: StoreType,
    file_id: &str,
    content_str: &str,
) -> Result<(), ApiError> {
    let tower_server = config.get_string(App::TowerServer);
    let (_, login_user) = get_login_user(&webview, &tower_server).await?;
    let mut file: StoreFile = bincode_decode(content_str);
    if file.user_id.ne(&login_user.user_id) {
        return Err(ApiError::Custom("这不是你的。".to_string()));
    }

    file.file_size = content_str.len() as u64;
    let file_path = get_file_path(&webview, store_type, &login_user.user_id, file_id);
    write_all(file_path, &bincode_encode(file)).unwrap();

    Ok(())
}

fn get_file_path(
    webview: &tauri::WebviewWindow,
    store_type: StoreType,
    user_id: &str,
    file_id: &str,
) -> String {
    let file_name = get_store_name_by_type(webview, store_type, format!("{}-{}", user_id, file_id));
    let dir = data_dir(webview).join(store_type.to_string().to_lowercase());
    if !dir.exists() {
        dir::create_all(&dir, false).unwrap();
    }
    let file_name = dir.join(file_name);
    file_name.to_string_lossy().to_string()
}

fn get_root_file(
    webview: &tauri::WebviewWindow,
    store_type: StoreType,
    user_id: &str,
) -> ApiResult<StoreRoot> {
    let root = get_file_path(&webview, store_type, user_id, STORE_ROOT_ID);
    if !Path::new(&root).exists() {
        let content = StoreRoot::new(user_id);
        write_all(root, &bincode_encode(&content)).unwrap();
        return Ok(content);
    }
    read_bin_file(&root)
}

#[cfg(test)]
mod tests {
    use tower::{
        common::bincode_decode,
        common_rs::read_bin_file,
        tauri_model::{note::NoteDir, StoreFile, StoreRoot},
    };

    #[test]
    fn test_filename() {
        let config = "C:\\Users\\gelye\\AppData\\Roaming\\assitant\\note\\q3r-tzApbeqqFxzThN6rr8YY9cLl14dqccds84YpdHdddXeddd3C";
        println!("{:?}", read_bin_file::<StoreRoot>(config));
    }

    #[test]
    fn test_filename_data() {
        let config = "C:\\Users\\gelye\\AppData\\Roaming\\fifth-tower\\note\\WddXdddddmNv0PCznqz8rrxcyZsQ9snperC_3Es8drA__HFm0pedFH";
        println!("{:?}", read_bin_file::<StoreFile>(config));
    }
    #[test]
    fn test_store_file() {
        let config = "iWDDDxiWxidqxedeWdsddXddcddddddddddwdqedKued-tmd0ddd1ddHCdddLmdeexddd3dnCCxOidcsddddddbdddWx";
        println!("{:?}", bincode_decode::<StoreFile>(config));
    }
    #[test]
    fn test_dirs() {
        let config =
            "ddd0ddd-dd8ddudCdddCpdideddHzddqdxdDdd9vdddddddCddddddddddddddddddemcdHdddeddoddp1";
        println!("{:?}", bincode_decode::<Vec<NoteDir>>(config));
    }
}
