use crate::db_models::*;
use diesel::prelude::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sqlite::SqliteConnection;
use diesel::mysql::MysqlConnection;
use std::env;

enum DbType {
    Postgres,
    Sqlite,
    Mysql,
}

#[derive(Default)]
pub struct Database {
    pub pg_connection: Option<PgConnection>,
    pub sqlite_connection: Option<PgConnection>,
    pub mysql_connection: Option<MysqlConnection>,
}

pub fn connect_postgres() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn connect_sqlite() -> SqliteConnection {
    let database_path = env::var("DATABASE_PATH")
        .expect("DATABASE_PATH must be set");
    SqliteConnection::establish(&database_path)
        .expect(&format!("Error connecting to {}", database_path))
}

pub fn connect_mysql() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
