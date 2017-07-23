extern crate stockism;
extern crate diesel;
extern crate serde_json;

use std::io::Read;

use self::stockism::*;
use self::diesel::prelude::*;

use self::diesel::pg::PgConnection;

use iron::status;
use iron::{Request, Response, IronResult};
use iron::mime::Mime;

pub fn list(_req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    use stockism::schema::warehouses::dsl::*;

    let connection = establish_connection();
    let results = warehouses
        .limit(10)
        .load::<Warehouse>(&*connection)
        .expect("Error loading warehouses");

    let warehouses_serialized = serde_json::to_string(&results).unwrap();

    Ok(Response::with((content_type, status::Ok, warehouses_serialized)))
}

use self::models::{Warehouse, NewWarehouse};

pub fn create(req: &mut Request) -> IronResult<Response> {
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let connection   = establish_connection();
    let mut payload  = String::new();
    req.body.read_to_string(&mut payload);
    match serde_json::from_str(&payload) {
        Ok(request) => {
            let warehouse: NewWarehouse = request;
            match create_warehouse(&connection, warehouse.name) {
                Ok(_warehouse) => Ok(Response::with((content_type, status::Ok,
                                                    "{'sucess': true}"))),
                Err(error)    => Ok(Response::with((content_type, status::Ok,
                                                    format!("{{'sucess': false, message: {} }}", error)))),
            }
        },
        Err(p_error) => Ok(Response::with((content_type, status::Ok,
                                           format!("{{'sucess': false, message: {} }}", p_error)))),
    }

}

pub fn create_warehouse<'a>(conn: &PgConnection, name: &'a str) -> QueryResult<Warehouse> {
    use self::schema::warehouses;

    let new_warehouse = NewWarehouse {
        scoped_id: 1,
        name: name,
    };

    diesel::insert(&new_warehouse).into(warehouses::table).get_result(conn)
}
