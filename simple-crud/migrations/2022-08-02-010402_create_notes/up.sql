-- Your SQL goes here

CREATE TABLE notes (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	body TEXT NOT NULL
)
