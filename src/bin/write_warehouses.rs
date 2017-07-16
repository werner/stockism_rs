extern crate stockism;
extern crate diesel;

use self::stockism::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Insert warehouse name");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)];

    let warehouse = create_warehouse(&connection, name);
    println!("{} warehouse saved", warehouse.name);

}
