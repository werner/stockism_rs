-- Your SQL goes here
CREATE TABLE products_warehouses(
    quantity DOUBLE PRECISION NOT NULL,
    product_id   INT REFERENCES products(id),
    warehouse_id INT REFERENCES warehouses(id)
);
CREATE UNIQUE INDEX products_warehouses_unique_idx ON products_warehouses (product_id, warehouse_id);
