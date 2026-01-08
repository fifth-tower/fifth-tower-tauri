use super::*;
use tauri::{Manager, Runtime};
use tower::{
    common::{user::RefreshReq, ApiError, StoreKey, TowerResource, Urlable},
    jwt::{JwtString, Principal, Token},
    reqwest::async_post_bin_and,
};
use tracing::debug;

pub(crate) fn get_token<T, R>(app: &T) -> Result<String, ApiError>
where
    T: Manager<R>,
    R: Runtime,
{
    get_store_value(app, StoreKey::AccessToken).ok_or(ApiError::UnAuthorite)
}

pub(crate) fn set_token<T, R>(app: &T, token: String) -> Result<(), ApiError>
where
    T: Manager<R>,
    R: Runtime,
{
    set_store_value(app, StoreKey::AccessToken, token)
}
pub(crate) fn set_refresh_token<T, R>(app: &T, token: String) -> Result<(), ApiError>
where
    T: Manager<R>,
    R: Runtime,
{
    set_store_value(app, StoreKey::RefreshToken, token)
}

pub(crate) async fn refresh_token<T, R>(
    app: &T,
    tower_server: &str,
    token: String,
) -> Result<(String, Principal), ApiError>
where
    T: Manager<R>,
    R: Runtime,
{
    //获取refresh_token
    let refresh_token =
        get_store_value(app, StoreKey::RefreshToken).ok_or(ApiError::UnAuthorite)?;
    if JwtString::try_from(refresh_token.clone()).is_err() {
        return Err(ApiError::UnAuthorite);
    }
    //refresh
    let url = TowerResource::Auth.url(tower_server, "/refresh");
    let token: String = async_post_bin_and(
        &url,
        &RefreshReq {
            access: token,
            refresh: refresh_token,
        },
        None,
    )
    .await?;
    let token = Token(token);
    let principal = token
        .get_attrs()
        .map(|attr| Principal::from(attr))
        .map_err(|_| ApiError::UnAuthorite)?;

    debug!("after refresh:{}", token.0);
    set_token(app, token.0.clone()).unwrap();
    Ok((token.0, principal))
}
