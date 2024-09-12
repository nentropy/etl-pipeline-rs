//! Main Binary where a session is established and all constant or other 
//! values are started in context
//! Parameters:
//!     UUID: Standard UUID for the ession
//!     Datetime: Start of the session
//!     Length: Length of the session: 
//! 1. Check if sesioon exists
mod enums;
mod structs; 
mod session_ctx;

use session_ctx::{ SessionContext, SessionID, Session }

async fn main() {
    //establish session

}