use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sqlite::SqliteConnection;
use diesel::mysql::MysqlConnection;
use diesel::connection::SimpleConnection;
use failure::Error;
use time;

use crate::db_models::*;
use crate::db_schema::*;
use crate::hash::{encode, decode};
use crate::conf::Conf;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

fn connect_sqlite(conf: &Conf) -> SqliteConnection {
    let database_path = &conf.database.path;
    let connection = SqliteConnection::establish(&database_path)
        .expect(&format!("Error connecting to {}", database_path));
    connection.batch_execute(INIT_SQLITE).unwrap();
    connection
}

const INIT_POSTGRES: &str = "
";

fn connect_postgres(conf: &Conf) -> PgConnection {
    let database_url = &conf.database.path;
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

const INIT_MYSQL: &str = "
";

fn connect_mysql(conf: &Conf) -> MysqlConnection {
    let database_url = &conf.database.path;
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn timestamp() -> String {
    time::now().to_local().ctime().to_string()
}

pub fn insert_url(conf: &Conf, url: &str) -> Result<String, Error> {
    let connection = connect_sqlite(conf);

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

pub fn get_url(conf: &Conf, hash: &str) -> Result<String, Error> {
    let connection = connect_sqlite(conf);

    let id = match decode(hash) {
        Some(h) => h,
        None => bail!("can't decode hash '{}'", hash),
    };

    let result = urls::table.filter(urls::id.eq(id))
        .limit(1)
        .load::<Url>(&connection)?;

    if result.len() == 1 {
        let hits = result[0].hits + 1;

        diesel::update(urls::table.find(id))
            .set(urls::hits.eq(&hits))
            .execute(&connection)
            .unwrap();

        diesel::update(urls::table.find(id))
            .set(urls::accessed.eq(&timestamp()))
            .execute(&connection)
            .unwrap();
    };


    warn!("{}", result.len());
    match result.len() {
        1 => Ok(result[0].url.clone()),
        _ => bail!("can't find entry for '{}' (id {})", hash, id),
    }
}
