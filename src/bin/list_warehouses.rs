extern crate stockism;
extern crate diesel;

use self::stockism::*;
use self::stockism::models::*;
use self::diesel::prelude::*;

fn main() {
    use stockism::schema::warehouses::dsl::*;

    let connection = establish_connection();
    let results = warehouses
        .limit(10)
        .load::<Warehouse>(&connection)
        .expect("Error loading warehouses");

    println!("Displaying {} warehouses", results.len());
    for warehouse in results {
        println!("{}", warehouse.scoped_id);
        println!("--------------\n");
        println!("{}", warehouse.name);
    }

}
