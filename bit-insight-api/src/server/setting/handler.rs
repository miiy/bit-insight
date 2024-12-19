use super::{service::Service, validate::validate_key};
use crate::auth::provider::AuthenticatedUser;
use crate::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use serde_json::Value;

// GET /settings/{key}
pub async fn detail(
    path: web::Path<String>,
    user: web::ReqData<AuthenticatedUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // validate key
    let key = path.into_inner();
    validate_key(&key).map_err(APIError::from)?;

    let resp = Service::detail(user.id, &key, &app_state.db)
        .await
        .map_err(APIError::from)?;

    Ok(HttpResponse::Ok().json(resp.value))
}

// PUT /settings/{key}
pub async fn update(
    path: web::Path<String>,
    req: web::Json<Value>,
    user: web::ReqData<AuthenticatedUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // validate key
    let key = path.into_inner();
    validate_key(&key).map_err(APIError::from)?;

    let resp = Service::update(user.id, &key, req.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
