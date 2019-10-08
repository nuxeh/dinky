#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub url: String,
    pub created: String,
    pub accessed: String,
    pub hits: i32,
}
