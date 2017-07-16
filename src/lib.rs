#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Warehouse, NewWarehouse};

pub fn create_warehouse<'a>(conn: &PgConnection, name: &'a str) -> Warehouse {
    use schema::warehouses;

    let new_warehouse = NewWarehouse {
        scoped_id: 1,
        name: name,
    };

    diesel::insert(&new_warehouse).into(warehouses::table)
        .get_result(conn)
        .expect("Error saving new warehouse")
}
