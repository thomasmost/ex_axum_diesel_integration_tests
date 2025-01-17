-- Your SQL goes here

DROP TABLE IF EXISTS `org`;
CREATE TABLE org (
  `id` char(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
    `name` varchar(199) NOT NULL,
    `created` bigint NOT NULL,
    PRIMARY KEY (`id`)
);
