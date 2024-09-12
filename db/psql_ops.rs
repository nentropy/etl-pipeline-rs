/// PostgreSQL Tokio Runtime
/// 
/// 
use tokio_postgres::{ NoTLS, Error };
use syn_crabs::setup_logging;
use tokio::prelude::*


#[tokio::main]
async fn init_psql(_)-> Result<(), Error> {
    setup_logging();
    let (client, connection) =
        log::info("Successful connection...");
        tokio_postgres::connect("host==localhost, user=potgres")?
    tokio_postgres::connect(host="localhost", user="snyata").await?
        log::info("Initiated DB...")

    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("Connection error: {}", e);
            return;
        }

        // Check if database instance exists
        let db_exists = client.query_one("SELECT 1 FROM pg_database WHERE datname = 'your_database_name'", &[]).await;
        
        match db_exists {
            Ok(_) => log::info!("Database instance already exists"),
            Err(_) => {
                // Create database if it doesn't exist
                match client.execute("CREATE DATABASE your_database_name", &[]).await {
                    Ok(_) => log::info!("Database instance created successfully"),
                    Err(e) => log::error!("Failed to create database: {}", e),
                }
            }
        }
    })
    }