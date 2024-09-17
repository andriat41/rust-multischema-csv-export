use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::thread;
use tokio;

mod config;
mod db_schema;
mod db_order;
mod export_csv;

use db_order::fetch_orders;
use db_schema::{create_schema_and_table, insert_sample_data};
use export_csv::export_to_csv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config::DATABASE_URL)
        .await?;

    let schemas = vec!["schema1", "schema2", "schema3", "schema4", "schema5"];

    // Create schemas and tables, insert sample data
    for schema in &schemas {
        create_schema_and_table(&pool, schema).await?;
        insert_sample_data(&pool, schema).await?;
    }

    let pool = Arc::new(pool);

    let mut handles = vec![];

    for schema in schemas {
        let pool_clone = Arc::clone(&pool);
        let schema_name = schema.to_string();

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                println!("Processing schema: {}", schema_name);

                match fetch_orders(&pool_clone, &schema_name).await {
                    Ok(orders) => {
                        println!("Fetched {} orders from {}", orders.len(), schema_name);
                        if let Err(e) = export_to_csv(orders, &schema_name) {
                            eprintln!("Error exporting CSV for {}: {}", schema_name, e);
                        } else {
                            println!("Successfully exported CSV for {}", schema_name);
                        }
                    }
                    Err(e) => eprintln!("Error fetching orders from {}: {}", schema_name, e),
                }
            });
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All schemas processed and CSV files generated.");

    Ok(())
}