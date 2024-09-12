use worker::*;
use serde_json::json;
use chrono::Utc;
use syn_crabs::setup_logging;
use uuid::Uuid::{Uuid4};


use crate::{ThreatData, OpenCTIClient};

pub struct Session { 
	uuid: Uuid4,
	timestamp: Utc::now()
}

impl Session {
    pub fn begin_session() -> Result<Session(uuid), <Err()>>
}

pub async fn handle_data_ingestion(mut req: Request, env: Env) -> Result<Response> {
	

    let threat_data: ThreatData = match req.json().await {
        Ok(data) => data,
        Err(_) => return Response::error("Invalid JSON", 400),
    };

    if !validate_threat_data(&threat_data) {
        return Response::error("Invalid threat data", 400);
    }

    // Store in D1 Database
    store_in_d1(&threat_data, &env).await?;

    // Cache in KV Storage
    cache_in_kv(&threat_data, &env).await?;

    // Push to OpenCTI
    push_to_opencti(&threat_data, &env).await?;

    Response::ok("Data ingested successfully")
}

fn validate_threat_data(data: &ThreatData) -> bool {
    // Implement validation logic
    !data.id.is_empty() && !data.threat_type.is_empty()
}


async fn store_in_d1(data: &ThreatData, env: &Env) -> Result<()> {
    let d1 = env.d1("THREAT_DB")?;
    let query = "INSERT INTO threat_data (id, type, attributes) VALUES (?, ?, ?)";
    let attributes = serde_json::to_string(&data.attributes)?;
    
    d1.prepare(query)
        .bind(&[&data.id, &data.threat_type, &attributes])?
        .run()
        .await?;

    Ok(())
}


/// Asynchronously caches threat data in the key-value store.
/// 
/// # Arguments
/// 
///  `data` - The threat data to be cached.
///  `env` - The environment configuration.
/// 
/// # Returns
/// 
/// A `Result` indicating success or an error.
/// 
/// # Errors
/// 
/// This function can return an error if there are issues with serializing the data or executing the cache operation.
/// 
/// # Examples
/// 
/// ```
/// # use crate::{ThreatData, Env};
/// # async fn example_usage() {
/// #     let data = ThreatData { id: "123".to_string() };
/// #     let env = Env::default();
/// #     let result = cache_in_kv(&data, &env).await;
/// #     assert!(result.is_ok());
/// # }
/// #
async fn cache_in_kv(data: &ThreatData, env: &Env) -> Result<()> {
    let kv = env.kv("THREAT_CACHE")?;
    let json_data = serde_json::to_string(&data)?;
    kv.put(&data.id, json_data)?.expiration_ttl(3600).execute().await?;
    Ok(())
}


/// Asynchronously pushes threat data to OpenCTI.
/// 
/// # Arguments
/// 
/// * `data` - A reference to the ThreatData struct containing the data to push.
/// * `env` - A reference to the Env struct providing access to secrets like the OpenCTI API URL and key.
/// 
/// # Returns
/// 
/// A Result indicating success or an error if the operation fails.
async fn push_to_opencti(data: &ThreatData, env: &Env) -> Result<()> {
    let opencti_url = env.secret("OPENCTI_API_URL")?.to_string();
    let opencti_key = env.secret("OPENCTI_API_KEY")?.to_string();
    let client = OpenCTIClient::new(opencti_url, opencti_key);

    client.create_stix_cyber_observable(data).await?;

    Ok(())
}