use std::fs::File;
use std::io::Write;
use crate::db_order::Order;

pub fn export_to_csv(orders: Vec<Order>, schema_name: &str) -> std::io::Result<()> {
    let filename = format!("{}_orders.csv", schema_name);
    let mut file = File::create(filename)?;

    writeln!(file, "id,customer_name,order_date,total_amount,schema_name")?;

    for order in orders {
        writeln!(
            file,
            "{},{},{},{},{}",
            order.id,
            order.customer_name,
            order.order_date,
            order.total_amount,
            order.schema_name
        )?;
    }

    Ok(())
}