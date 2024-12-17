use super::model::Setting;
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

impl Setting {
    pub async fn find(
        pool: &MySqlPool,
        user_id: u64,
        key: &str,
    ) -> Result<Option<Setting>, sqlx::Error> {
        let item: Option<Setting> = sqlx::query_as(
            "SELECT `id`, `user_id`, `key`, `value`, `created_at`, `updated_at`
            FROM `user_settings`
            WHERE `user_id`=? AND `key`=?
            ",
        )
        .bind(user_id)
        .bind(key)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }

    pub async fn create(
        pool: &MySqlPool,
        user_id: u64,
        key: &str,
        value: &str,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "INSERT INTO `user_settings` (`user_id`, `key`, `value`, `created_at`, `updated_at`)
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(user_id)
        .bind(key)
        .bind(value)
        .bind(OffsetDateTime::now_utc())
        .bind(OffsetDateTime::now_utc())
        .execute(pool)
        .await
        .map(|x| x.last_insert_id())
    }

    pub async fn update(
        pool: &MySqlPool,
        user_id: u64,
        key: &str,
        value: &str,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE `user_settings` SET `value`=?, `updated_at`=?
        WHERE `user_id`=? AND `key`=?
        ",
        )
        .bind(value)
        .bind(OffsetDateTime::now_utc())
        .bind(user_id)
        .bind(key)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
