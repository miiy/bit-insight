-- Add down migration script here
START TRANSACTION;

DROP TABLE IF EXISTS `materials`;

COMMIT;