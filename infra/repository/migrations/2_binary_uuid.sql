-- CREATE `*v2` TABLEs --

CREATE TABLE IF NOT EXISTS `owners_v2` (
    `id` BINARY(16) NOT NULL PRIMARY KEY,
    `name` VARCHAR(30) NOT NULL,
    `kind` ENUM('group', 'single_user') NOT NULL
);

CREATE TABLE IF NOT EXISTS `groups_v2` (
    `id` BINARY(16) NOT NULL PRIMARY KEY,
    `name` VARCHAR(30) NOT NULL
);

CREATE TABLE IF NOT EXISTS `users_v2` (
    `id` BINARY(16) NOT NULL PRIMARY KEY,
    `name` VARCHAR(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS `group_members_v2` (
    `group_id` BINARY(16) NOT NULL,
    `user_id` BINARY(16) NOT NULL,
    PRIMARY KEY (`group_id`, `user_id`),
    FOREIGN KEY (`group_id`) REFERENCES `groups_v2` (`id`) ON DELETE CASCADE,
    FOREIGN KEY (`user_id`) REFERENCES `users_v2` (`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `webhooks_v2` (
    `id` BINARY(16) NOT NULL PRIMARY KEY,
    `channel_id` BINARY(16) NOT NULL,
    `owner_id` BINARY(16) NOT NULL,
    FOREIGN KEY (`owner_id`) REFERENCES `owners_v2` (`id`) ON DELETE CASCADE
);

-- INSERT INTO `*v2` tables --

INSERT IGNORE INTO
    `owners_v2` (`id`, `name`, `kind`)
SELECT
    UNHEX(REPLACE(`id`, '-', '')) AS `id`,
    `name`,
    IF(`group`, 'group', 'single_user') AS `kind`
FROM
    `owners`;

INSERT IGNORE INTO
    `groups_v2` (`id`, `name`)
SELECT
    UNHEX(REPLACE(`id`, '-', '')) AS `id`,
    `name`
FROM
    `groups`;

INSERT IGNORE INTO
    `users_v2` (`id`, `name`)
SELECT
    UNHEX(REPLACE(`id`, '-', '')) AS `id`,
    `name`
FROM
    `users`;

INSERT IGNORE INTO
    `group_members_v2` (`group_id`, `user_id`)
SELECT
    UNHEX(REPLACE(`group_id`, '-', '')) AS `group_id`,
    UNHEX(REPLACE(`user_id`, '-', '')) AS `user_id`
FROM
    `group_members`;

INSERT IGNORE INTO
    `webhooks_v2` (`id`, `channel_id`, `owner_id`)
SELECT
    UNHEX(REPLACE(`id`, '-', '')) AS `id`,
    UNHEX(REPLACE(`channel_id`, '-', '')) AS `channel_id`,
    UNHEX(REPLACE(`owner_id`, '-', '')) AS `owner_id`
FROM
    `webhooks`;

-- DELETE old rows --

DELETE FROM `webhooks`;

DELETE FROM `group_members`;

DELETE FROM `owners`;

DELETE FROM `groups`;

DELETE FROM `users`;
