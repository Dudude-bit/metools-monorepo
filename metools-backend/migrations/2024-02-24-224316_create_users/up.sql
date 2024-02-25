-- Your SQL goes here
CREATE TABLE `users`(
	`id` INT4 NOT NULL PRIMARY KEY,
	`title` VARCHAR NOT NULL,
	`body` TEXT NOT NULL,
	`published` BOOL NOT NULL
);

