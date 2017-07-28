extern crate stockism;
extern crate diesel;
extern crate serde_json;

use std::io::Read;
use std::error::Error;

use self::diesel::prelude::*;

use iron::{Request, Response, IronResult};

use handlers::utils::*;
use middlewares::DieselReqExt;
use models::{Warehouse, NewWarehouse, EditWarehouse};

pub fn list(req: &mut Request) -> IronResult<Response> {
    use stockism::schema::warehouses::dsl::*;

    let connection = req.get_db_conn();
    let results = warehouses
        .limit(10)
        .load::<Warehouse>(&*connection)
        .expect("Error loading warehouses");

    response_ok(&results)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
    let body = get_body!(req, response_bad_request);

    match serde_json::from_str::<NewWarehouse>(&body) {
        Ok(new_warehouse) => {
            match Warehouse::create(&connection, &new_warehouse) {
                Ok (_warehouse) => response_ok_success(),
                Err(error)      => response_internal_server_error(error.to_string()),
            }
        },
        Err(error)        => response_bad_request(format!("{}: {}", error.description(), error))
    }
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
    let body = get_body!(req, response_bad_request);

	let warehouse_id = get_route_parameter_as!(i32, req, "id", response_not_found("Warehouse not found"));

    match serde_json::from_str::<EditWarehouse>(&body) {
        Ok(edit_warehouse) => {
            match Warehouse::update(&connection, warehouse_id, &edit_warehouse) {
                Ok (_warehouse) => response_ok_success(),
                Err(error)      => response_internal_server_error(error.to_string()),
            }
        },
        Err(error)        => response_bad_request(format!("{}: {}", error.description(), error))
    }

}
