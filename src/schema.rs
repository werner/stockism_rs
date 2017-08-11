infer_schema!("dotenv:DATABASE_URL");

table! {
    product_types (id) {
        id -> Int4,
        scoped_id -> Nullable<Int4>,
        name -> Varchar,
    }
}
