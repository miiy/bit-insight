use super::{dto::*, service::Service};
use crate::error::APIError;
use crate::jwt::AuthenticatedUser;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// GET /settings/key
pub async fn detail(
    path: web::Path<String>,
    user: web::ReqData<AuthenticatedUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = Service::detail(&path.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// PUT /settings
pub async fn update(
    request: web::Json<UpdateRequest>,
    user: web::ReqData<AuthenticatedUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = Service::update(&request.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
