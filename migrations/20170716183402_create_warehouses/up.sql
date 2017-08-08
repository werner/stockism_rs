-- Your SQL goes here
CREATE TABLE warehouses (
    id SERIAL PRIMARY KEY,
    scoped_id INT,
    name VARCHAR NOT NULL UNIQUE,
    CHECK (name <> '')
);
