use sqlx::{FromRow, PgPool};
use std::fmt::Debug;

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub description: String
}

pub async fn create_task(pool: &PgPool, name: &str, description: Option<&str>) -> Result<(),
sqlx::Error> {
    sqlx::query("INSERT INTO tasks (name, description) VALUES ($1, $2)")
        .bind(name)
        .bind(description)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_tasks(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    let tasks = sqlx::query_as::<_, Todo>("SELECT id, name, description FROM tasks")
        .fetch_all(pool)
        .await?;

    Ok(tasks)
}
