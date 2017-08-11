extern crate stockism;
extern crate serde_json;

use std::io::Read;
use std::error::Error;

use iron::prelude::*;
use iron::{Request, Response, IronResult};

use handlers::utils::*;
use middlewares::DieselReqExt;
use models::{ProductType, NewProductType, EditProductType};

pub fn list(req: &mut Request) -> IronResult<Response> {
    use params::Params;
    let connection = req.get_db_conn();

    let default_limit  = 10;
    let default_offset = 0;
    let map = req.get_ref::<Params>().unwrap();

	let limit = get_params_argument!(map,  &["limit"],  i64, default_limit);
	let offset = get_params_argument!(map, &["offset"], i64, default_offset);

    match ProductType::list(&connection, limit, offset) {
        Ok (_product_types) => response_ok(&_product_types),
        Err(error)       =>  response_internal_server_error(error.to_string()),
    }
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
	let product_type_id = get_route_parameter_as!(i32, req, "id", response_not_found("You should provide an id"));

    match ProductType::edit(&connection, product_type_id) {
        Ok (_product_type) => response_ok(&_product_type),
        Err(error)      => response_internal_server_error(error.to_string()),
    }
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
    let body = get_body!(req, response_bad_request);

    let new_product_type = get_body_as!(NewProductType, &body, req, response_bad_request);

    match ProductType::create(&connection, &new_product_type) {
        Ok (_product_type) => response_ok_success(),
        Err(error)      => response_internal_server_error(error.to_string()),
    }
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
	let product_type_id = get_route_parameter_as!(i32, req, "id", response_not_found("Product Type not found"));
    
    let body = get_body!(req, response_bad_request);
    let edit_product_type = get_body_as!(EditProductType, &body, req, response_bad_request);

    match ProductType::update(&connection, product_type_id, &edit_product_type) {
        Ok (_product_type) => response_ok_success(),
        Err(error)      => response_internal_server_error(error.to_string()),
    }

}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
	let product_type_id = get_route_parameter_as!(i32, req, "id", response_not_found("Product Type not found"));
    
    match ProductType::delete(&connection, product_type_id) {
        Ok (_product_type) => response_ok_success(),
        Err(error)      => response_internal_server_error(error.to_string()),
    }

}
