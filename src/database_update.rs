use rocket_sync_db_pools::diesel;
use rocket_sync_db_pools::diesel::connection::SimpleConnection;
use std::fs;

pub struct DatabaseUpdater {

}

impl DatabaseUpdater {

    fn update(dbConnection: diesel::PgConnection){

        dbConnection.build_transaction()
            .read_only()
            .run::<_,diesel::result::Error,_>(
                || {
                    let read_attempt = meta_info.select(version)
                }
            )?;
    }

    fn find_next_script(db_connection: diesel::PgConnection){
        println!("In file {}", filename);

        let index:i8 = 0;

        while let contents = fs::read_to_string(format!("db_update_{}.sql", index)).unwrap() {
            db_connection.batch_execute(&contents);
        }

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        println!("With text:\n{}", contents);
    }
}