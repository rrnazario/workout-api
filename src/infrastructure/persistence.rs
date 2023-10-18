use std::env;

use diesel_async::{AsyncPgConnection, AsyncConnection};

pub async fn connect_async() -> AsyncPgConnection {
    let conn_string = env::var("DATABASE_URL").expect("DATABASE_URL is empty.");

    AsyncPgConnection::establish(&conn_string)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}