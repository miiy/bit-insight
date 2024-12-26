use super::model::{Material, MaterialStatus};
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

impl Material {
    pub async fn create(pool: &MySqlPool, item: &Material) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO materials (`user_id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `status`, `created_at`, `updated_at`)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(&item.user_id)
        .bind(&item.category_id)
        .bind(&item.title)
        .bind(&item.author)
        .bind(&item.source)
        .bind(&item.source_url)
        .bind(&item.thumbnail)
        .bind(&item.summary)
        .bind(&item.content)
        .bind(&item.status)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(pool)
        .await
        .map(|x| x.last_insert_id())
    }

    pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Material>, sqlx::Error> {
        let item: Option<Material> = sqlx::query_as(
            "
        SELECT `id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `created_at`, `updated_at`
        FROM `materials`
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }

    pub async fn find_count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM materials WHERE deleted_at IS NULL")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    pub async fn find_all(
        pool: &MySqlPool,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Material>, sqlx::Error> {
        let items: Vec<Material> = sqlx::query_as(
            "
        SELECT `id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `status`, `created_at`, `updated_at`
        FROM `materials`
        WHERE `status` = ? AND `deleted_at` IS NULL
        ORDER BY `created_at` DESC
        LIMIT ? OFFSET ?
        ",
        )
        .bind(MaterialStatus::Published.as_i8())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn update(pool: &MySqlPool, item: &Material) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE materials SET `category_id` = ?, `title` = ?, `author` = ?, `source` = ?, `source_url` = ?, `thumbnail` = ?, `summary` = ?, `content` = ?, `updated_at` = ?
        WHERE `id`=?
        ",
        )
            .bind(&item.category_id)
            .bind(&item.title)
            .bind(&item.author)
            .bind(&item.source)
            .bind(&item.source_url)
            .bind(&item.thumbnail)
            .bind(&item.summary)
            .bind(&item.content)
            .bind(&item.updated_at)
            .bind(&item.id)
            .execute(pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn soft_delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE materials SET `deleted_at` = ?
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(OffsetDateTime::now_utc())
        .bind(id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
