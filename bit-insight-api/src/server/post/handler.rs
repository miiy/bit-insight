use super::dto;
use super::error::PostError;
use super::service::Service;
use crate::error::APIError;
use crate::server::setting::service::Service as SettingService;
use crate::server::user::service::Service as UserService;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use std::collections::HashMap;

// GET /posts
pub async fn list(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string())?;
    let page = query
        .get("page")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);
    let resp = Service::lists(page, page_size, &app_state.db)
        .await
        .map_err(APIError::from)?;

    Ok(HttpResponse::Ok().json(resp))
}

// GET /posts/{id}
pub async fn detail(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(PostError::Params(e.to_string())))?;
    let resp = Service::detail(id, &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// POST /posts
pub async fn create(
    params: web::Json<dto::CreateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = Service::create(params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Created().json(resp))
}

// PUT /posts/{id}
pub async fn update(
    path: web::Path<String>,
    params: web::Json<dto::UpdateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(PostError::Params(e.to_string())))?;
    let resp = Service::update(id, params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// DELETE /posts/{id}
pub async fn delete(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(PostError::Params(e.to_string())))?;
    let resp = Service::delete(id, &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// POST /posts/push
pub async fn push(
    req: HttpRequest,
    params: web::Json<dto::PushRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    // validate header token
    let push_username = req
        .headers()
        .get("Push-Username")
        .ok_or_else(|| APIError::from(PostError::Unauthorized))?
        .to_str()
        .map_err(|_| APIError::from(PostError::Unauthorized))?;
    let push_token = req
        .headers()
        .get("Push-Token")
        .ok_or_else(|| APIError::from(PostError::Unauthorized))?
        .to_str()
        .map_err(|_| APIError::from(PostError::Unauthorized))?;

    let user = UserService::user(&push_username, &app_state.db)
        .await
        .map_err(|_| APIError::from(PostError::Unauthorized))?;

    let setting = SettingService::detail(user.id, "push", &app_state.db)
        .await
        .map_err(|_| APIError::from(PostError::Unauthorized))?;
    let setting_push_token = setting
        .value
        .get("push_token")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if push_token != setting_push_token {
        return Err(APIError::from(PostError::Unauthorized).into());
    }

    let resp = Service::push(params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
