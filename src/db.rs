use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sqlite::SqliteConnection;
use diesel::mysql::MysqlConnection;
use diesel::connection::SimpleConnection;
use crate::db_models::*;
use crate::db_schema::*;
use time;
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

pub fn insert_url(url: &str) {
    let connection = connect_sqlite();

    let id = urls::table
        .count()
        .execute(&connection)
        .unwrap_or(0) as i32;

    /*
    let hits = urls::table.filter(urls::id.eq(id))
        .limit(1)
        .load::<Url>(&connection)
        .expect("Error loading posts");
*/

    let entry = NewUrl {
        id: id,
        url: url,
        created: &timestamp(),
        accessed: "",
        hits: 0,
    };

    diesel::insert_into(urls::table)
        .values(&entry)
        .execute(&connection)
        .expect("Error saving new post");


    //println!("{:?}", results);

}
