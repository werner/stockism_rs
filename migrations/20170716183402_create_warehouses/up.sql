-- Your SQL goes here
CREATE TABLE warehouses (
    id SERIAL PRIMARY KEY,
    scoped_id INT NOT NULL,
    name VARCHAR NOT NULL
)
