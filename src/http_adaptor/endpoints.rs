use router::Router;

use handlers::root_handler;
use handlers::warehouses_handler;
use handlers::product_types_handler;

macro_rules! set_default_routes {
	($router:expr, $plural:expr, $singular:expr, $handler:ident) => {
		{
            $router.get(format!("/{}", $plural), $handler::list, format!("{}", $singular));
            $router.get(format!("/{}/:id", $singular), $handler::edit, format!("edit_{}", $singular));
            $router.post(format!("/{}", $singular), $handler::create, format!("post_{}", $singular));
            $router.put(format!("/{}/:id", $singular), $handler::update, format!("put_{}", $singular));
            $router.delete(format!("/{}/:id", $singular), $handler::delete, format!("delete_{}", $singular));
            $router
		}
	}
}

pub fn declare_endpoints() -> Router {
    let mut router = Router::new();
    router.get("/", root_handler::handler, "root");
    router = set_default_routes!(router, "warehouses", "warehouse", warehouses_handler);
    set_default_routes!(router, "product_types", "product_type", product_types_handler)
}
