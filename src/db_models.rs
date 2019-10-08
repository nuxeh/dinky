use crate::db_schema::urls;

#[derive(Queryable)]
pub struct Url {
    pub id: i32,
    pub url: String,
    pub created: String,
    pub accessed: String,
    pub hits: i32,
}

#[derive(Insertable)]
#[table_name="urls"]
pub struct NewUrl<'a> {
    pub url: &'a str,
}
