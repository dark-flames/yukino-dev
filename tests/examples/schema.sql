CREATE TABLE person (
    `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `age` INT UNSIGNED NOT NULL,
    `level` SMALLINT UNSIGNED NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE meeting (
    `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(255) NOT NULL,
    `host_id` INT UNSIGNED NOT NULL,
    `co_host_id` INT UNSIGNED NOT NULL,
    `start_time` BIGINT UNSIGNED NOT NULL,
    `end_time` BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;