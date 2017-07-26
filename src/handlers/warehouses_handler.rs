extern crate stockism;
extern crate diesel;
extern crate serde_json;

use std::io::Read;
use std::error::Error;

use self::stockism::*;
use self::diesel::prelude::*;

use self::diesel::pg::PgConnection;

use iron::mime::Mime;
use iron::status;
use iron::{Request, Response, IronResult};

use handlers::utils::*;
use middlewares::DieselReqExt;

pub fn list(req: &mut Request) -> IronResult<Response> {
    use stockism::schema::warehouses::dsl::*;

    let connection = req.get_db_conn();
    let results = warehouses
        .limit(10)
        .load::<Warehouse>(&*connection)
        .expect("Error loading warehouses");

    response_ok(&results)
}

use self::models::{Warehouse, NewWarehouse};

pub fn create(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
    let body = get_body!(req, response_bad_request);

    match serde_json::from_str::<NewWarehouse>(&body) {
        Ok(new_warehouse) => {
            match create_warehouse(&connection, &new_warehouse.name) {
                Ok (_warehouse) => response_ok_success(),
                Err(error)      => response_internal_server_error(error.to_string()),
            }
        },
        Err(error)        => response_bad_request(format!("{}: {}", error.description(), error))
    }

    
}

fn create_warehouse<'a>(conn: &PgConnection, name: &'a str) -> QueryResult<Warehouse> {
    use self::schema::warehouses;

    let new_warehouse = NewWarehouse {
        scoped_id: Some(get_last_scoped_id()),
        name: name,
    };

    diesel::insert(&new_warehouse).into(warehouses::table).get_result(conn)
}

fn get_last_scoped_id() -> i32 {
    use stockism::schema::warehouses::dsl::*;
    let connection = establish_connection();
    let results = warehouses
        .limit(1)
        .order(scoped_id.desc())
        .load::<Warehouse>(&*connection)
        .expect("Error loading warehouses");
    let mut _scoped_id: i32 = 0;
    for warehouse in results {
        _scoped_id = warehouse.scoped_id.unwrap_or(0) + 1;
    }
    if _scoped_id == 0 {
        1
    } else {
        _scoped_id
    }
}
