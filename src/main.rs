//! Main Binary where a session is established and all constant or other 
//! values are started in context
//! Parameters:
//!     UUID: Standard UUID for the ession
//!     Datetime: Start of the session
//!     Length: Length of the session: 
//! 1. Check if sesioon exists
mod enums;
mod structs;
mod data_ingestion;
mod data_transformation;

pub use data_ingestion::DataIngestionService;
pub use data_transformation::DataTransformationService; 

use syn_crabs::setup_logging;
use session_ctx::{ SessionContext, SessionID, Session };
use redis::{ RedisConnectionInfo, ReddisError, RedisClient }

enum FileTypes {
    Csv,
    Json,
    Parquet,
    Streaming,
}

impl FileTypes {
 matches! {
    if csv =>
 }

}


pub fn main() -> Result<(), Box<dyn RedisError<()>> {
    let redis_client = RedisClient::new("redis://127.0.0.1"?);
    let mut conn = redis_client.get_connection()?;

   
    //Data Ingestion
    let raw_data = data_ingestion::ingest_data()

    match setup_logging(verbose=false, quiet=false) {
        Ok(_) => {
            log::info!("Logging setup successful");
            Ok(())
        },
        Err(e) => {
            log::error!("Failed to load logging: {:?}", e);
            Err(Error::new(()))
        }
    }
}



}