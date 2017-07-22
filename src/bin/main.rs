extern crate stockism;
extern crate iron;
#[macro_use]extern crate router;

use iron::status;
use iron::{Iron, Request, Response, IronResult};

mod handlers;

fn main() {
    let router = router! {
        root: get "/" => handler,
        warehouses: get "/warehouses" => handlers::warehouses_handler::list_handler
    };

    Iron::new(router).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "/")))
}
