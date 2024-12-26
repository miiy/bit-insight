use super::dto;
use super::error::MaterialError;
use super::service::Service;
use crate::error::APIError;

use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use std::collections::HashMap;

// GET /materials
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

// GET /materials/{id}
pub async fn detail(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(MaterialError::Params(e.to_string())))?;
    let resp = Service::detail(id, &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// POST /materials
pub async fn create(
    params: web::Json<dto::CreateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = Service::create(params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Created().json(resp))
}

// PUT /materials/{id}
pub async fn update(
    path: web::Path<String>,
    params: web::Json<dto::UpdateRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(MaterialError::Params(e.to_string())))?;
    let resp = Service::update(id, params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// DELETE /materials/{id}
pub async fn delete(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(MaterialError::Params(e.to_string())))?;
    let resp = Service::delete(id, &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}

// POST /materials/push
pub async fn push(
    req: HttpRequest,
    params: web::Json<dto::PushRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    // validate header token
    let push_username = req
        .headers()
        .get("Push-Username")
        .ok_or_else(|| APIError::from(MaterialError::Unauthorized))?
        .to_str()
        .map_err(|_| APIError::from(MaterialError::Unauthorized))?;

    let push_token = req
        .headers()
        .get("Push-Token")
        .ok_or_else(|| APIError::from(MaterialError::Unauthorized))?
        .to_str()
        .map_err(|_| APIError::from(MaterialError::Unauthorized))?;

    let resp = Service::push(push_username, push_token, params.into_inner(), &app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
