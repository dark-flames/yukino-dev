CREATE TABLE IF NOT EXISTS user(
    `id`           INT               NOT NULL AUTO_INCREMENT,
    `name`         VARCHAR(31)       NOT NULL,
    `age`          SMALLINT UNSIGNED NOT NULL,
    `phone`        VARCHAR(15)       NOT NULL,
    `address`      VARCHAR(31)       NOT NULL,
    `birthday`     DATE              NOT NULL,
    `since`        DATETIME          NOT NULL,
    `introduction` TEXT              NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS examination (
    `id`         INT    NOT NULL AUTO_INCREMENT,
    `user_id`    INT    NOT NULL,
    `start_time` BIGINT NOT NULL,
    `end_time`   BIGINT NOT NULL,
    `comment`    TEXT   NOT NULL,
    PRIMARY KEY (`id`),
    CONSTRAINT `user_exam` FOREIGN KEY (`user_id`) REFERENCES user (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

