use super::dto::*;
use super::error::ArticleError;
use super::model::{Article, ArticleStatus};
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
    ) -> Result<ListResponse, ArticleError> {
        let total = Article::find_count(&pool).await?;
        let pg = Pagination::new(page, per_page, total);

        let offset = pg.offset();
        let articles = Article::find_all(&pool, pg.per_page, offset).await?;
        let results: Vec<ListResponseItem> = articles
            .into_iter()
            .map(|article| ListResponseItem {
                id: article.id,
                category_id: article.category_id,
                title: article.title,
                author: article.author,
                source: article.source,
                source_url: article.source_url,
                thumbnail: article.thumbnail,
                summary: article.summary,
                created_at: article
                    .created_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                updated_at: article
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

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<DetailResponse, ArticleError> {
        let article = Article::find(&pool, id).await?.ok_or(ArticleError::NotFound)?;

        let resp = DetailResponse {
            id: article.id,
            category_id: article.category_id,
            title: article.title,
            author: article.author,
            source: article.source,
            source_url: article.source_url,
            thumbnail: article.thumbnail,
            summary: article.summary,
            content: article.content,
            created_at: article
                .created_at
                .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            updated_at: article
                .updated_at
                .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
        };
        Ok(resp)
    }

    pub async fn create(req: CreateRequest, pool: &MySqlPool) -> Result<CreateResponse, ArticleError> {
        let now = OffsetDateTime::now_utc();
        let article = Article {
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
            status: ArticleStatus::Published.as_i8(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        let article_id = Article::create(&pool, &article).await?;

        let resp = CreateResponse { id: article_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<UpdateResponse, ArticleError> {
        let article = Article {
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
            status: ArticleStatus::Published.as_i8(),
            created_at: None,
            updated_at: Some(OffsetDateTime::now_utc()),
        };
        let article_id = Article::update(&pool, &article).await?;

        Ok(UpdateResponse { id: article_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<DeleteResponse, ArticleError> {
        let _ra = Article::soft_delete(&pool, id).await?;
        Ok(DeleteResponse {})
    }

    pub async fn push(push_username: &str, push_token: &str, req: PushRequest, pool: &MySqlPool) -> Result<PushResponse, ArticleError> {
        let user = UserService::user(&push_username, pool)
        .await
        .map_err(|_| ArticleError::Unauthorized)?;

        let setting = SettingService::detail(user.id, "push", pool)
            .await
            .map_err(|_| ArticleError::Unauthorized)?;

        let setting_push_token = setting
            .value
            .get("push_token")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if push_token != setting_push_token {
            return Err(ArticleError::Unauthorized)
        }

        let now = OffsetDateTime::now_utc();
        let article = Article {
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
            status: ArticleStatus::Published.as_i8(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        let article_id = Article::create(&pool, &article).await?;

        let resp = PushResponse { id: article_id };
        Ok(resp)
    }
}
