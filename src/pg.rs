#![allow(dead_code)]
use r2d2::{ Pool, Config };
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::PooledConnection;
use postgres::rows::Row;
use postgres::error::Error;
use postgres_shared::error::{SqlState};
use postgres_shared::types::ToSql;

use config::Config as AppConfig;
use errors::*;

pub fn create_db_pool(app_config: &AppConfig) -> Pool<PostgresConnectionManager> {
    let database_url = app_config.database_url.clone();
    let config = Config::default();
    let manager = PostgresConnectionManager::new(database_url, TlsMode::None).expect("Create PostgresConnectionManager error");
    Pool::new(config, manager).expect("Failed to create pool")
}

pub trait Insertable {
    fn insert_query(&self) -> String;
    fn insert_params(&self) -> Box<[&ToSql]>;
}

pub trait Existable {
    fn exist_query() -> String;
}

pub struct PgDatabase {
    connection: PooledConnection<PostgresConnectionManager>,
}

impl PgDatabase {
    pub fn new(connection: PooledConnection<PostgresConnectionManager>) -> Self {
        PgDatabase { connection }
    }

    pub fn insert<E>(&self, entity: &E) -> Result<u64> where E: Insertable {
        self.connection.execute(&entity.insert_query(), &entity.insert_params())
        .map_err(|e| {
            println!("{:?}", e);
            match e {
                Error::Db(ref e) if e.code == SqlState::UniqueViolation => ErrorKind::AlreadyExist.into(),
                e => e.into(),
            }
        })
    }

    pub fn exist<'a>(&self, query: &str, params: &[&'a ToSql]) -> Result<bool> {
        let rows = self.connection.query(query, params)?;
        Ok(rows.iter().fold(false, |_, row| {
            let exist: i64 = row.get("exist");
            exist > 0
        }))
    }

    pub fn find<'a, E>(&self, query: &str, params: &[&'a ToSql]) -> Result<Vec<E>> where E: for<'b> From<Row<'b>> {
        let rows = self.connection.query(query, params)?;
        Ok(rows.iter().map(|row| row.into()).collect())
    }

    pub fn find_one<'a, E>(&self, query: &str, params: &[&'a ToSql]) -> Result<Option<E>> where E: for<'b> From<Row<'b>> {
        let rows = self.connection.query(query, params)?;
        let mut items: Vec<E> = rows.iter().map(|row| row.into()).collect();
        Ok(items.pop())
    }
}
