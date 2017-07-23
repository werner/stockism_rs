#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use]extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate dotenv;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub type DieselConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type DieselPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DieselConnection {
    get_pool().get().expect("Failed to get db connection")
}

fn get_pool() -> DieselPool {
    dotenv().ok();

    let config = r2d2::Config::default();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool")
}
