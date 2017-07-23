extern crate stockism;
extern crate diesel;
extern crate serde_json;

use self::stockism::*;
use self::stockism::models::*;
use self::diesel::prelude::*;

use iron::status;
use iron::{Request, Response, IronResult};
use iron::mime::Mime;

pub fn list_handler(req: &mut Request) -> IronResult<Response> {
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

