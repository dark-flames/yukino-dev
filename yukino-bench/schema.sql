CREATE TABLE IF NOT EXISTS user(
    `id`  INT NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(31) NOT NULL,
    `age` SMALLINT UNSIGNED NOT NULL,
    `phone` VARCHAR(15) NOT NULL,
    `address` VARCHAR(31) NOT NULL,
    `birthday` DATE NOT NULL,
    `since` DATETIME NOT NULL,
    `introduction` TEXT,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;