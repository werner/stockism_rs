extern crate stockism;
extern crate serde_json;

use std::io::Read;
use std::error::Error;

use iron::prelude::*;
use iron::{Request, Response, IronResult};

use handlers::utils::*;
use middlewares::DieselReqExt;
use models::{Warehouse, NewWarehouse, EditWarehouse};

pub fn list(req: &mut Request) -> IronResult<Response> {
    use params::Params;
    let connection = req.get_db_conn();

    let default_limit  = 10;
    let default_offset = 0;
    let map = req.get_ref::<Params>().unwrap();

	let limit = get_params_argument!(map,  &["limit"],  I64, &default_limit);
	let offset = get_params_argument!(map, &["offset"], I64, &default_offset);

    match Warehouse::list(&connection, *limit, *offset) {
        Ok (_warehouses) => response_ok(&_warehouses),
        Err(error)       =>  response_internal_server_error(error.to_string()),
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
	let warehouse_id = get_route_parameter_as!(i32, req, "id", response_not_found("You should provide an id"));

    match Warehouse::edit(&connection, warehouse_id) {
        Ok (_warehouse) => response_ok(&_warehouse),
        Err(error)      => response_internal_server_error(error.to_string()),
    }
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
    let body = get_body!(req, response_bad_request);

    let new_warehouse = get_body_as!(NewWarehouse, &body, req, response_bad_request);

    match Warehouse::create(&connection, &new_warehouse) {
        Ok (_warehouse) => response_ok_success(),
        Err(error)      => response_internal_server_error(error.to_string()),
    }
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
	let warehouse_id = get_route_parameter_as!(i32, req, "id", response_not_found("Warehouse not found"));
    
    let body = get_body!(req, response_bad_request);
    let edit_warehouse = get_body_as!(EditWarehouse, &body, req, response_bad_request);

    match Warehouse::update(&connection, warehouse_id, &edit_warehouse) {
        Ok (_warehouse) => response_ok_success(),
        Err(error)      => response_internal_server_error(error.to_string()),
    }

}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
	let warehouse_id = get_route_parameter_as!(i32, req, "id", response_not_found("Warehouse not found"));
    
    match Warehouse::delete(&connection, warehouse_id) {
        Ok (_warehouse) => response_ok_success(),
        Err(error)      => response_internal_server_error(error.to_string()),
    }

}
