use super::schema::warehouses;

#[derive(Queryable)]
pub struct Warehouse {
    pub id: i32,
    pub scoped_id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="warehouses"]
pub struct NewWarehouse<'a> {
    pub scoped_id: i32,
    pub name: &'a str,
}
