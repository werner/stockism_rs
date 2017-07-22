extern crate iron;
#[macro_use]extern crate router;

use iron::status;
use iron::{Iron, Request, Response, IronResult, AfterMiddleware, Chain};
use iron::error::{IronError};
use router::{NoRoute};

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Hitting custom 404 middleware");

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "404 Error, Page not Found.")))
        } else {
            Err(err)
        }
    }
}

fn main() {
    let router = router! {
        root: get "/" => handler,
        warehouses: get "/warehouses" => warehouses_handler
    };

    let mut chain = Chain::new(router);
    chain.link_after(Custom404);

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "/")))
}

fn warehouses_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "warehouses")))
}
