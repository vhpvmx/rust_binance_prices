CREATE TABLE `test`.`prices` ( `id` INT NOT NULL AUTO_INCREMENT , `symbol` VARCHAR(255) NULL DEFAULT NULL , `datetime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP , `price` FLOAT NOT NULL DEFAULT '0' , PRIMARY KEY (`id`)) ENGINE = InnoDB;