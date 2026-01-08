use std::fs;

use tauri::{Manager, Runtime};
use tauri_plugin_store::StoreExt;
use tower::{
    tauri_model::StoreType,
    common::{bincode_dec, bincode_encode, ApiError, App},
    common_rs::time_encryt,
};
use tracing::error;

fn get_store_name<T, R, B>(app: &T, prefix: B) -> String
where
    T: Manager<R>,
    R: Runtime,
    B: ToString,
{
    let local_dir = app.path().app_local_data_dir().unwrap();
    let metadata = fs::metadata(local_dir).unwrap();
    let created = metadata.created().unwrap();

    time_encryt(prefix, created)
}

pub(crate) fn get_user_store_name<T, R>(app: &T) -> String
where
    T: Manager<R>,
    R: Runtime,
{
    get_store_name(app, App::TowerTauri)
}

pub(crate) fn get_store_name_by_type<T, R, B>(
    app: &T,
    store_type: StoreType,
    file_index: B,
) -> String
where
    T: Manager<R>,
    R: Runtime,
    B: ToString,
{
    get_store_name(
        app,
        format!("{}/{}", store_type.to_string(), file_index.to_string()),
    )
}

pub(crate) fn get_store_value<T, R, K>(app: &T, name: K) -> Option<String>
where
    T: Manager<R>,
    R: Runtime,
    K: ToString,
{
    let store_name = get_user_store_name(app);
    let store = app.store(store_name.clone());
    store
        .ok()
        .map(|st| {
            st.get(&name.to_string())
                .map(|f| {
                    f.as_str().and_then(|f| {
                        bincode_dec::<(String, String)>(f).map_or(None, |(prefix, token)| {
                            if store_name.eq(&prefix) {
                                Some(token)
                            } else {
                                None
                            }
                        })
                    })
                })
                .flatten()
        })
        .flatten()
}

pub(crate) fn set_store_value<T, R, K>(app: &T, name: K, value: String) -> Result<(), ApiError>
where
    T: Manager<R>,
    R: Runtime,
    K: ToString,
{
    let store_name = get_user_store_name(app);
    let store = app.store(store_name.clone());
    store
        .map(|store| {
            store.set(name.to_string(), bincode_encode((store_name, value)));
        })
        .map_err(|err| {
            error!("set_token:{:?}", err);
            ApiError::BadClient
        })
}

pub(crate) fn delete_store<T, R, K>(app: &T, name: K) -> Result<(), ApiError>
where
    T: Manager<R>,
    R: Runtime,
    K: ToString,
{
    let store_name = get_user_store_name(app);
    let store = app.store(store_name.clone());
    store
        .map(|store| {
            store.delete(name.to_string());
        })
        .map_err(|err| {
            error!("set_token:{:?}", err);
            ApiError::BadClient
        })
}
