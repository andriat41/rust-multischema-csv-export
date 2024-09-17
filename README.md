# Rust Multi-Schema Database Processor

## Overview

This project demonstrates a Rust application that processes data from multiple database schemas concurrently and exports the results to CSV files. It showcases Rust's powerful features for concurrent programming, database interaction using SQLx, and file I/O operations.

#RustMultithreading #DatabaseParallelProcessing #SQLxPostgreSQL #CSVExport #MultiSchemaArchitecture

## Features

- Dynamic schema and table creation in PostgreSQL
- Concurrent data fetching from multiple schemas
- Efficient CSV export of data from each schema
- Modular design with clear separation of concerns
- Use of SQLx for database operations
- Tokio for asynchronous runtime

## Project Structure

```
src/
├── main.rs           # Application entry point and main logic
├── config.rs         # Configuration settings
├── db_schema.rs      # Database schema operations
├── db_order.rs       # Order struct and database queries
├── export_csv.rs     # CSV export functionality
Cargo.toml            # Project dependencies and metadata
```

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-multischema-processor.git
   cd rust-multischema-processor
   ```

2. Update the database connection string in `src/config.rs`:
   ```rust
   pub const DATABASE_URL: &str = "postgres://username:password@localhost/database_name";
   ```

3. Install dependencies:
   ```
   cargo build
   ```

## Running the Application

Execute the following command in the project root:

```
cargo run
```

The application will:
1. Create multiple schemas in the specified database
2. Insert sample data into each schema
3. Fetch data from all schemas concurrently
4. Export data from each schema to separate CSV files

## Customization

- Modify the `schemas` vector in `main.rs` to change the number or names of schemas processed.
- Adjust the `Order` struct in `db_order.rs` to change the data structure.
- Modify the CSV export format in `export_csv.rs` to change the output format.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).
