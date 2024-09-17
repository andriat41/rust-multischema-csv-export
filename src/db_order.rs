use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Order {
    pub id: i32,
    pub customer_name: String,
    pub order_date: NaiveDateTime,
    pub total_amount: f64,
    pub schema_name: String,
}

pub async fn fetch_orders(pool: &PgPool, schema_name: &str) -> Result<Vec<Order>, sqlx::Error> {
    let query = format!(
        r#"
        SELECT
            id,
            customer_name,
            order_date,
            total_amount,
            '{schema_name}' as schema_name
        FROM {schema_name}.orders
        "#,
        schema_name = schema_name
    );

    let orders = sqlx::query(&query)
        .bind(schema_name)
        .map(|row: PgRow| Order {
            id: row.get("id"),
            customer_name: row.get("customer_name"),
            order_date: row.get("order_date"),
            total_amount: row.get("total_amount"),
            schema_name: row.get("schema_name"),
        })
        .fetch_all(pool)
        .await?;

    Ok(orders)
}