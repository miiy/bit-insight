use super::{service::Service, error::SettingError};
use crate::error::APIError;
use crate::jwt::AuthenticatedUser;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use serde_json::Value;

fn validate_key(key: &str) -> Result<(), SettingError> {
    if key.is_empty() {
        return Err(SettingError::KeyNotFound("".into()));
    }
    Ok(())
}

// GET /settings/key
pub async fn detail(
    path: web::Path<String>,
    user: web::ReqData<AuthenticatedUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // validate key
    let key = path.into_inner();
    validate_key(&key).map_err(APIError::from)?;

    let user_id = user.user_id;

    let resp = Service::detail(user_id, &key, &app_state.db)
        .await
        .map_err(APIError::from)?;

    Ok(HttpResponse::Ok().json(resp.value))
}


// PUT /settings/key
pub async fn update(
    path: web::Path<String>,
    req: web::Json<Value>,
    user: web::ReqData<AuthenticatedUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // validate key
    let key = path.into_inner();
    validate_key(&key).map_err(APIError::from)?;

    let user_id = user.user_id;

    let resp = Service::update(user_id, &key, req.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
