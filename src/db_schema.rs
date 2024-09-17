use sqlx::postgres::PgPool;

pub async fn create_schema_and_table(pool: &PgPool, schema_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(
        "DROP TABLE IF EXISTS {}.orders",
        schema_name
    ))
    .execute(pool)
    .await?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {}.orders (
            id SERIAL PRIMARY KEY,
            customer_name TEXT NOT NULL,
            order_date TIMESTAMP NOT NULL,
            total_amount FLOAT8 NOT NULL
         )",
        schema_name
    ))
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_sample_data(pool: &PgPool, schema_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(
        "INSERT INTO {}.orders (customer_name, order_date, total_amount) VALUES
         ('Alice', '2023-01-01 10:00:00', 100.50),
         ('Bob', '2023-01-02 11:00:00', 200.75),
         ('Charlie', '2023-01-03 12:00:00', 150.25)",
        schema_name
    ))
    .execute(pool)
    .await?;

    Ok(())
}