use std::fs;
use crate::models::MessageQueue;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::r2d2::{Pool, PooledConnection};
use r2d2_postgres::postgres::NoTls;

pub struct DatabaseUpdater {

}

impl DatabaseUpdater {

    pub(crate) fn update(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>){
        let current_version =DatabaseUpdater::check_current_version(connection);
    }

    fn check_current_version(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> i32 {
        let query = connection.prepare("SELECT db_version FROM meta_info");

        let current_version = connection.query(&query, &[])
            .iter()
            .fold(None, |row| {
                row.get(0).unwrap()
            });
        current_version[0]
    }

    fn update_db_version(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>, next_version: i8) {
        let query = connection.prepare("UPDATE meta_info SET db_version = $1");
        connection.query(&query, &[&next_version]);
    }

    fn find_next_script(connection: &mut PooledConnection<PostgresConnectionManager<NoTls>>){

        let index:i8 = 0;

        while let contents = fs::read_to_string(format!("db_update_{}.sql", index)).unwrap() {
            connection.batch_execute(&contents);
        }
    }
}