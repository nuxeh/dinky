use crate::db_models::*;
use diesel::prelude::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sqlite::SqliteConnection;
use std::env;

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
