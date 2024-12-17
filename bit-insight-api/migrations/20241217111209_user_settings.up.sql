-- Add up migration script here
START TRANSACTION;

CREATE TABLE IF NOT EXISTS `user_settings`
(
    `id`                 bigint unsigned NOT NULL AUTO_INCREMENT,
    `user_id`            bigint unsigned NOT NULL,
    `key`                varchar(255)    NOT NULL DEFAULT '',
    `value`              json            NOT NULL,
    `created_at`         timestamp       NULL     DEFAULT NULL,
    `updated_at`         timestamp       NULL     DEFAULT NULL,
    `deleted_at`         timestamp       NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `user_settings_user_id_index` (`user_id`),
    KEY `user_settings_key_index` (`key`)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;

COMMIT;
