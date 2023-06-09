use log::error;
use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

// proceed to configure db connection
// postgreSQL is bounded by RAM in number of actice connection it can handle,
// 20 is safe
pub async fn connect(database_url: &str) -> Result<Pool<Postgres>, crate::Error> {
    PgPoolOptions::new()
        .max_connections(50)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 min
        .connect(database_url)
        .await
        .map_err(|err| {
            error!("😈😈😈😈😈😈db: connecting to you DB :) 😈😈: {}", err);
            err.into()
        })
}

pub async fn migrate(db: &Pool<Postgres>) -> Result<(), crate::Error> {
    match sqlx::migrate!("./db/migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("db::migrate: migrating: {}", &err);
            Err(err)
        }
    }?;
    Ok(())
}

/* 
#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn test_db_connection() {
        let db = connect("postgres://h3cker:postgres@localhost:5432/zkratlolz")
        .await
        .unwrap();
    
    dbg!(&db);
    
    // db is not closed
    assert_eq!(db.is_closed(), false);
}

}
 */