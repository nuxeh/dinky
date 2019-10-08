use crate::db_schema::urls;

#[derive(Queryable, Debug)]
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
    pub id: i32,
    pub url: &'a str,
    pub created: &'a str,
    pub accessed: &'a str,
    pub hits: i32,
}
