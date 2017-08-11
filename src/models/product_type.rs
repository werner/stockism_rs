extern crate stockism;
extern crate diesel;

use stockism::schema::product_types;
use self::diesel::prelude::*;
use self::diesel::pg::PgConnection;

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct ProductType {
    pub id: i32,
    pub scoped_id: Option<i32>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="product_types"]
pub struct NewProductType<'a> {
    pub scoped_id: Option<i32>,
    pub name: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct EditProductType {
    pub name: String,
}

impl ProductType {

    pub fn list(conn: &PgConnection, limit: i64, offset: i64) -> QueryResult<Vec<ProductType>> {
        use stockism::schema::product_types::dsl::*;
        product_types
            .limit(limit)
            .offset(offset)
            .order(scoped_id.asc())
            .load::<ProductType>(&*conn)
    }

    pub fn edit(conn: &PgConnection, id: i32) -> QueryResult<ProductType> {
        use stockism::schema::product_types::dsl::product_types;
        product_types.find(id).first::<ProductType>(&*conn)
    }

    pub fn create<'a>(conn: &PgConnection, new_product_type: &'a NewProductType) -> QueryResult<ProductType> {
        use stockism::schema::product_types;
        use diesel;

        let last_scoped_id = ProductType::get_last_scoped_id(conn);

        let new_product_type = NewProductType {
            scoped_id: Some(last_scoped_id),
            name: new_product_type.name,
        };

        diesel::insert(&new_product_type).into(product_types::table).get_result(conn)
    }

    pub fn update<'a>(conn: &PgConnection, id: i32, 
                      product_type: &'a EditProductType) -> QueryResult<ProductType> {
        use stockism::schema::product_types::dsl::{product_types, name};
        use diesel;

        diesel::update(product_types.find(id))
            .set(name.eq(product_type.name.clone()))
            .get_result(conn)
    }

    pub fn delete(conn: &PgConnection, id: i32) -> QueryResult<ProductType> {
        use stockism::schema::product_types::dsl::product_types;
        use diesel;

        diesel::delete(product_types.find(id)).get_result(conn)
    }

    fn get_last_scoped_id(conn: &PgConnection) -> i32 {
        use stockism::schema::product_types::dsl::*;
        get_last_scoped_id!(conn, product_types, ProductType, scoped_id.desc())
    }

}

