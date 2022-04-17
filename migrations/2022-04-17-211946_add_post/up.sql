-- Your SQL goes here

CREATE TABLE posts (
  id SERIAL NOT NULL PRIMARY KEY,
  content varchar NOT NULL,
  title varchar(255) NOT NULL,
  user_id INT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now(),
  deleted_at TIMESTAMP DEFAULT NULL,
  CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id)
);
