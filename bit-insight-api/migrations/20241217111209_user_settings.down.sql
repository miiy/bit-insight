-- Add down migration script here
START TRANSACTION;

DROP TABLE IF EXISTS `user_settings`;

COMMIT;