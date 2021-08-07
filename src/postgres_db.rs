

#[database("pg_user_db")]
struct DbConn(diesel::PgConnection);