extern crate stockism;
extern crate diesel;

use stockism::schema::warehouses;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Warehouse {
    pub id: i32,
    pub scoped_id: Option<i32>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="warehouses"]
pub struct NewWarehouse<'a> {
    pub scoped_id: Option<i32>,
    pub name: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct EditWarehouse {
    pub name: String,
}

impl Warehouse {

    pub fn create<'a>(conn: &PgConnection, new_warehouse: &'a NewWarehouse) -> QueryResult<Warehouse> {
        use stockism::schema::warehouses;
        use diesel;

        let last_scoped_id = Warehouse::get_last_scoped_id(conn);

        let new_warehouse = NewWarehouse {
            scoped_id: Some(last_scoped_id),
            name: new_warehouse.name,
        };

        diesel::insert(&new_warehouse).into(warehouses::table).get_result(conn)
    }

    pub fn update<'a>(conn: &PgConnection, id: i32, 
                      warehouse: &'a EditWarehouse) -> QueryResult<Warehouse> {
        use stockism::schema::warehouses::dsl::{warehouses, name};
        use diesel;

        diesel::update(warehouses.find(id))
            .set(name.eq(warehouse.name.clone()))
            .get_result(conn)

    }

    fn get_last_scoped_id(conn: &PgConnection) -> i32 {
        use stockism::schema::warehouses::dsl::*;
        get_last_scoped_id!(conn, warehouses, Warehouse, scoped_id.desc())
    }

}
