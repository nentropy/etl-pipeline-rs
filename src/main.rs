//! Main Binary where a session is established and all constant or other 
//! values are started in context
//! Parameters:
//!     UUID: Standard UUID for the ession
//!     Datetime: Start of the session
//!     Length: Length of the session: 
//! 1. Check if sesioon exists
mod enums;
mod structs; 

use syn_crabs::setup_logging;
use session_ctx::{ SessionContext, SessionID, Session };


pub fn main() -> Result<(), Error<()>> {
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