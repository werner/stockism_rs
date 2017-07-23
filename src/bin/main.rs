extern crate stockism;
extern crate iron;
#[macro_use]extern crate router;

use iron::status;
use iron::{Iron, Request, Response, IronResult};

mod handlers;

fn main() {
    let router = router! {
        root: get "/" => handler,
        warehouses: get "/warehouses" => handlers::warehouses_handler::list,
        post_warehouse: post "/warehouse" => handlers::warehouses_handler::create
    };

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "/")))
}
