-- Your SQL goes here

CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email varchar(320) NOT NULL,
  username varchar(32) NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),
  deleted_at TIMESTAMP DEFAULT NULL
);
