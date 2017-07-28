use router::Router;

use handlers::root_handler;
use handlers::warehouses_handler;

pub fn declare_endpoints() -> Router {
    router! {
        root: get "/" => root_handler::handler,
        warehouses: get "/warehouses" => warehouses_handler::list,
        post_warehouse: post "/warehouse" => warehouses_handler::create,
        put_warehouse: put "/warehouse/:id" => warehouses_handler::update,
        delete_warehouse: delete "/warehouse/:id" => warehouses_handler::delete
    }
}
