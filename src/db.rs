use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

pub async fn connect_db() -> Result<PgPool, sqlx::Error>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE url must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to the database!");
    Ok(pool)
}
