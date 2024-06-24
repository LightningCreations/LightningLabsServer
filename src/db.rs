use std::env;

use diesel_async::{AsyncConnection, AsyncPgConnection};

pub async fn establish_connection() -> AsyncPgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
