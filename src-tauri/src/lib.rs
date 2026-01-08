// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
mod command;
mod common;

use std::sync::{Arc, Mutex};

use command::*;
use flow::{Dir, FLowResource, Recording};
use fs_extra::dir;
use tauri::Manager;
use tauri_plugin_store::StoreExt;
use tower::{
    common::{dict::Dict, App},
    config_client::get_public_config,
};
use tracing::debug;

use crate::{api::get_dict_by_code, common::get_user_store_name};

fn init_dir() {
    dir::create_all(Dir::Record.path(), false).unwrap();
    // dir::create_all(Dir::Script.path(), false).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            record::start_record,
            record::end_record,
            record::cancel_record,
            record::set_hotkeys,
            record_manager::update_app_setting,
            record_manager::delete_record_flow,
            record_manager::save_record_flow,
            record_manager::get_record_apps,
            record_manager::clone_flow,
            record_manager::delete_app,
            vip_check::check_flow_num,
            vip_check::check_wishing_num,
            vip_check::get_vip_level_item_num,
            auth::get_login_info,
            auth::login,
            auth::register,
            auth::change_password,
            auth::logout,
            user_info::get_user_apps,
            script::upload_script,
            script::download_script,
            script::get_installed_scripts,
            script::get_scripts_info,
            script::delete_script,
            instance::test_flow,
            instance::test_action_config,
            instance::stop_instance,
            instance::execute_scripts,
            instance::check_window,
            window::get_windows,
            public::get_control_keys,
            public::get_public_asset,
            public::open_url_by_brower,
            public::open_url,
            http::http,
            http::http_without_token,
            http::http_bin,
            http::http_bin_without_token,
            store::load_stores,
            store::load_store,
            store::create_store,
            store::delete_store,
            store::save_store,
            store::set_store_extra_info,
            store::get_store_extra_info
        ])
        .setup(|app: &mut tauri::App| {
            let config = tauri::async_runtime::block_on(async {
                get_public_config(&App::TowerTauri).await.unwrap()
            });
            let dict = get_dict_by_code(Dict::VipLevel);
            app.manage(Mutex::new(Recording::new()));
            app.manage(Arc::new(FLowResource::new()));
            app.manage(config);
            app.manage(dict);

            let app_dir = app.path().app_data_dir().unwrap();

            debug!("app_dir:{}", app_dir.display());
            flow::init_root_dir(&app_dir.to_string_lossy().to_string());
            init_dir();

            let _ = app.store(get_user_store_name(app))?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
