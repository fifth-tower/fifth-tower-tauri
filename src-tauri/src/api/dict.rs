use tower::{common::dict::DictData, config_client::get_dict_data};

pub fn get_dict_by_code<T>(dict: T) -> DictData
where
    T: Into<String>,
{
    tauri::async_runtime::block_on(async { get_dict_data(dict).await.unwrap() })
}
