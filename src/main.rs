extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate stockism;
extern crate iron;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;
#[macro_use]
extern crate router;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

mod handlers;
mod http_adaptor;
mod utils;
mod middlewares;

use http_adaptor::HttpAdaptor;
use utils::logger_factory;

fn main() {
    let logger = logger_factory();

	let mut http_server = HttpAdaptor::new(&logger);

	let routes = http_server.declare_endpoints();
	let chain = http_server.create_chain(routes);

    
    http_server.start_http(chain, "localhost", "3000");
}
