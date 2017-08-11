-- Your SQL goes here
CREATE TABLE products (
    id serial PRIMARY KEY,
    scoped_id INT,
    name VARCHAR NOT NULL UNIQUE,
    code VARCHAR UNIQUE,
    buying_price BIGINT, -- Representing cents
    stock DOUBLE PRECISION, 
    description TEXT,
    image_path VARCHAR,
    supplier_id INT REFERENCES suppliers(id),
    product_type_id INT REFERENCES product_types(id),
    CHECK (name <> '')
);
