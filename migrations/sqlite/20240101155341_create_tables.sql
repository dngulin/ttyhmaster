CREATE TABLE IF NOT EXISTS `players` (
    `player_name` varchar(32) NOT NULL PRIMARY KEY,
    `player_id` varchar(64) NOT NULL UNIQUE,
    `password_hash` varchar(64) NOT NULL,
    `salt` varchar(64) NOT NULL,
    `is_mojang` boolean NOT NULL DEFAULT FALSE,
    `is_slim_model` boolean NOT NULL DEFAULT FALSE,
    `access_token` varchar(64) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS `stats` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `player_name` varchar(32) NOT NULL,
    `launcher_ver` varchar(32) NOT NULL,
    `os` varchar(32) NOT NULL,
    `os_version` varchar(64) NOT NULL,
    `os_word_size` varchar(32) NOT NULL,
    `install_uuid` varchar(64) NOT NULL,
    `machine_uuid` varchar(64) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
);