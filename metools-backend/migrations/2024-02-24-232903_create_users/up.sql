-- Your SQL goes here
CREATE TABLE `users`(
	`id` UUID NOT NULL PRIMARY KEY,
	`title` VARCHAR NOT NULL,
	`body` TEXT NOT NULL,
	`published` BOOL NOT NULL
);

