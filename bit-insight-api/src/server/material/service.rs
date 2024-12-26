use super::dto::*;
use super::error::MaterialError;
use super::model::{Material, MaterialStatus};
use crate::pagination::Pagination;
use crate::server::setting::service::Service as SettingService;
use crate::server::user::service::Service as UserService;
use sqlx::MySqlPool;
use time::OffsetDateTime;


pub struct Service;

impl Service {
    pub async fn lists(
        page: u32,
        per_page: u32,
        pool: &MySqlPool,
    ) -> Result<ListResponse, MaterialError> {
        let total = Material::find_count(&pool).await?;
        let pg = Pagination::new(page, per_page, total);

        let offset = pg.offset();
        let materials = Material::find_all(&pool, pg.per_page, offset).await?;
        let results: Vec<ListResponseItem> = materials
            .into_iter()
            .map(|material| ListResponseItem {
                id: material.id,
                category_id: material.category_id,
                title: material.title,
                author: material.author,
                source: material.source,
                source_url: material.source_url,
                thumbnail: material.thumbnail,
                summary: material.summary,
                created_at: material
                    .created_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                updated_at: material
                    .updated_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            })
            .collect();

        Ok(ListResponse {
            page: pg.page,
            per_page: pg.per_page,
            total_pages: pg.total_pages,
            data: results,
        })
    }

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<DetailResponse, MaterialError> {
        let material = Material::find(&pool, id).await?.ok_or(MaterialError::NotFound)?;

        let resp = DetailResponse {
            id: material.id,
            category_id: material.category_id,
            title: material.title,
            author: material.author,
            source: material.source,
            source_url: material.source_url,
            thumbnail: material.thumbnail,
            summary: material.summary,
            content: material.content,
            created_at: material
                .created_at
                .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            updated_at: material
                .updated_at
                .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
        };
        Ok(resp)
    }

    pub async fn create(req: CreateRequest, pool: &MySqlPool) -> Result<CreateResponse, MaterialError> {
        let now = OffsetDateTime::now_utc();
        let material = Material {
            id: 0,
            user_id: 0,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            source: req.source,
            source_url: req.source_url,
            thumbnail: req.thumbnail,
            summary: req.summary,
            content: req.content,
            status: MaterialStatus::Published.as_i8(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        let material_id = Material::create(&pool, &material).await?;

        let resp = CreateResponse { id: material_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<UpdateResponse, MaterialError> {
        let material = Material {
            id: id,
            user_id: 0,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            source: req.source,
            source_url: req.source_url,
            thumbnail: req.thumbnail,
            summary: req.summary,
            content: req.content,
            status: MaterialStatus::Published.as_i8(),
            created_at: None,
            updated_at: Some(OffsetDateTime::now_utc()),
        };
        let material_id = Material::update(&pool, &material).await?;

        Ok(UpdateResponse { id: material_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<DeleteResponse, MaterialError> {
        let _ra = Material::soft_delete(&pool, id).await?;
        Ok(DeleteResponse {})
    }

    pub async fn push(push_username: &str, push_token: &str, req: PushRequest, pool: &MySqlPool) -> Result<PushResponse, MaterialError> {
        let user = UserService::user(&push_username, pool)
        .await
        .map_err(|_| MaterialError::Unauthorized)?;

        let setting = SettingService::detail(user.id, "push", pool)
            .await
            .map_err(|_| MaterialError::Unauthorized)?;

        let setting_push_token = setting
            .value
            .get("push_token")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if push_token != setting_push_token {
            return Err(MaterialError::Unauthorized)
        }

        let now = OffsetDateTime::now_utc();
        let material = Material {
            id: 0,
            user_id: user.id,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            source: req.source,
            source_url: req.source_url,
            thumbnail: req.thumbnail,
            summary: req.summary,
            content: req.content,
            status: MaterialStatus::Published.as_i8(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        let material_id = Material::create(&pool, &material).await?;

        let resp = PushResponse { id: material_id };
        Ok(resp)
    }
}
