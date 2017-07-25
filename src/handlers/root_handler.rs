use iron::status;
use iron::{Request, Response, IronResult};

pub fn handler(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "/")))
}

