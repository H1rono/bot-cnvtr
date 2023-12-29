CREATE TABLE IF NOT EXISTS `webhooks` (
    `id` CHAR(64) NOT NULL PRIMARY KEY,
    `channel_id` CHAR(36) NOT NULL,
    `owner_id` CHAR(36) NOT NULL
);

CREATE TABLE IF NOT EXISTS `owners` (
    `id` CHAR(36) NOT NULL PRIMARY KEY,
    `name` VARCHAR(30) NOT NULL,
    `group` BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS `groups` (
    `id` CHAR(36) NOT NULL PRIMARY KEY,
    `name` VARCHAR(30) NOT NULL
);

CREATE TABLE IF NOT EXISTS `users` (
    `id` CHAR(36) NOT NULL PRIMARY KEY,
    `name` VARCHAR(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS `group_members` (
    `group_id` CHAR(36) NOT NULL,
    `user_id` CHAR(36) NOT NULL,
    PRIMARY KEY (`group_id`, `user_id`),
    FOREIGN KEY (`group_id`) REFERENCES `groups` (`id`) ON DELETE CASCADE,
    FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
);
