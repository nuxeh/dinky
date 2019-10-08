use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sqlite::SqliteConnection;
use diesel::mysql::MysqlConnection;
use diesel::connection::SimpleConnection;
use crate::db_models::*;
use crate::db_schema::*;
use crate::hash::{encode, decode};
use time;
use failure::Error;

use std::env;

#[derive(Clone, Serialize, Deserialize)]
pub enum DbType {
    Sqlite,
    Postgres,
    Mysql,
}

impl Default for DbType {
    fn default() -> Self { DbType::Sqlite }
}

#[derive(Default)]
pub struct Database {
    pub pg_connection: Option<PgConnection>,
    pub sqlite_connection: Option<PgConnection>,
    pub mysql_connection: Option<MysqlConnection>,
}

const INIT_SQLITE: &str = "
CREATE TABLE IF NOT EXISTS urls (
    id              INTEGER,
    url             TEXT NOT NULL,
    created         TEXT NOT NULL,
    accessed        TEXT NOT NULL,
    hits            INTEGER
);";

fn connect_sqlite() -> SqliteConnection {
    let database_path = env::var("DATABASE_PATH")
        .expect("DATABASE_PATH must be set");
    let connection = SqliteConnection::establish(&database_path)
        .expect(&format!("Error connecting to {}", database_path));
    connection.batch_execute(INIT_SQLITE).unwrap();
    connection
}

const INIT_POSTGRES: &str = "
";

fn connect_postgres() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

const INIT_MYSQL: &str = "
";

fn connect_mysql() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn timestamp() -> String {
    time::now().to_local().ctime().to_string()
}

pub fn insert_url(url: &str) -> Result<String, Error> {
    let connection = connect_sqlite();

    let id = urls::table
        .count()
        .get_result(&connection)
        .unwrap_or(0) as i32;

    let entry = NewUrl {
        id: id,
        url: url,
        created: &timestamp(),
        accessed: "",
        hits: 0,
    };

    diesel::insert_into(urls::table)
        .values(&entry)
        .execute(&connection)?;

    match encode(id) {
        Some(h) => Ok(h),
        None => bail!("Can't encode hash for id {}", id),
    }
}

pub fn get_url(hash: &str) -> Result<String, Error> {
    let connection = connect_sqlite();

    let id = match decode(hash) {
        Some(h) => h,
        None => bail!("can't decode hash '{}'", hash),
    };

    let result = urls::table.filter(urls::id.eq(id))
        .limit(1)
        .load::<Url>(&connection)?;

    match result.len() {
        1 => Ok(result[0].url.clone()),
        _ => bail!("can't find entry for '{}' (id {})", hash, id),
    }
}
