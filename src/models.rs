#[derive(Queryable)]
pub struct Warehouse {
    pub id: i32,
    pub scoped_id: i32,
    pub name: String,
}
