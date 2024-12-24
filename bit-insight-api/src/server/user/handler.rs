use super::service::Service;
use super::vo::UserResponse;
use crate::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};

// GET /users/{username}
pub async fn user(
    username: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = Service::user(&username, &app_state.db)
        .await
        .map_err(APIError::from)?;

    let resp = UserResponse {
        username: resp.username,
        status: resp.status,
        created_at: resp.created_at,
        updated_at: resp.updated_at,
    };
    Ok(HttpResponse::Ok().json(resp))
}
