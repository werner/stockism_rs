use std::collections::HashSet;
use std::env;

use iron::prelude::*;
use router::Router;
use slog::Logger;

use http_adaptor::declare_endpoints;

use middlewares::DieselMiddleware;
use middlewares::LoggerMiddleware;

pub struct HttpAdaptor {
	logger: Logger
}

impl HttpAdaptor {
	pub fn new(logger: &Logger) -> HttpAdaptor {
		HttpAdaptor {logger: logger.new(o!("module" => "HttpAdaptor"))}
	}

	pub fn declare_endpoints(&mut self) -> Router {
		let routes = declare_endpoints();

		routes
	}

	pub fn create_chain(&self, routes: Router) -> Chain {
		let mut chain = Chain::new(routes);

		self.add_default_middlewares(&mut chain);

		chain
	}

	fn add_default_middlewares(&self, chain: &mut Chain) {
		let db_pool_middleware = DieselMiddleware::new(&self.logger);
		let logger_middleware = LoggerMiddleware::new(&self.logger);

		chain.link_before(logger_middleware);
		chain.link_before(db_pool_middleware);
	}

	pub fn start_http(&self, chain: Chain, host: &str, port: &str) {
		let address = format!("{}:{}", host, port);
		
		{
			info!(self.logger, "Server Running"; o!("address" => address.clone()));
		}

		Iron::new(chain).http(address).unwrap();
	}
}
