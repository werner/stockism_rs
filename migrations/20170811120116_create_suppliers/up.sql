-- Your SQL goes here
CREATE TABLE suppliers(
    id SERIAL PRIMARY KEY,
    scoped_id INT,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE,
    CHECK (first_name <> ''),
    CHECK (last_name <> '')
);
