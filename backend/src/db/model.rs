use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = super::schema::uploads)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Upload {
    pub id: String,
    pub filename: String,
    pub hash: String,
    pub size: i32,
    pub timestamp: i64,
}
