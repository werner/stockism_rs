extern crate serde;
extern crate serde_json;

use super::schema::warehouses;
use super::diesel::prelude::*;
use super::diesel::pg::PgConnection;

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

impl Warehouse {

    pub fn create<'a>(conn: &PgConnection, new_warehouse: &'a NewWarehouse) -> QueryResult<Warehouse> {
        use schema::warehouses;
        use diesel;

        let last_scoped_id = Warehouse::get_last_scoped_id(conn);

        let new_warehouse = NewWarehouse {
            scoped_id: Some(last_scoped_id),
            name: new_warehouse.name,
        };

        diesel::insert(&new_warehouse).into(warehouses::table).get_result(conn)
    }

    fn get_last_scoped_id(conn: &PgConnection) -> i32 {
        use schema::warehouses::dsl::*;
        warehouses
            .order(scoped_id.desc())
            .first::<Warehouse>(&*conn)
            .expect("Error loading warehouses")
            .scoped_id.unwrap_or(0) + 1
    }

}
