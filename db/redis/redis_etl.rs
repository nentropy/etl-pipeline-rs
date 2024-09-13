///REDIS FOR CACHING AND EXPLORATION
/// 
/// 
use redis::{Client, Commands, Connnection};
use serde::{ Deserialize, Serialize };
use std:error::Error;

//Data Structs
#[derive(Serialize, Deserialize)]
struct RawData {
    id: String
}