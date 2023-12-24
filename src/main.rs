use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::TcpListener;
use api::routes::handle_client;
use api::handlers::DB_URL;
use dotenv::dotenv;

#[macro_use]
extern crate serde_derive;

mod api;
mod utils;
mod models;

fn main() {
    dotenv().ok();

    // Set database
    if let Err(e) = set_database() {
        println!("Error setting database: {}", e);
        return;
    }

    // Start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8000")).unwrap();
    println!("Server started at port 8000");

    // handle the Postgres client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}


fn set_database() -> Result<(), PostgresError> {
    // Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Create table
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )"
    )?;
    Ok(())
}


