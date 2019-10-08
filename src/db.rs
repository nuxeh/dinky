use crate::db_models::*;
use diesel::prelude::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sqlite::SqliteConnection;
use diesel::mysql::MysqlConnection;
use crate::diesel::connection::SimpleConnection;
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

pub fn connect_sqlite() -> SqliteConnection {
    let database_path = env::var("DATABASE_PATH")
        .expect("DATABASE_PATH must be set");
    let connection = SqliteConnection::establish(&database_path)
        .expect(&format!("Error connecting to {}", database_path));
    connection.batch_execute(INIT_SQLITE).unwrap();
    connection
}

const INIT_POSTGRES: &str = "
";

pub fn connect_postgres() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

const INIT_MYSQL: &str = "
";

pub fn connect_mysql() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
