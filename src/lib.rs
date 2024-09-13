//! Cloudflare ETL Pipeline Data Collection
//! 
mod session_ctx;
mod enums;


use worker::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use syn_crabs::setup_logging;
use uuid::Uuid;
mod data_ingestion;
mod data_transformation;

pub use data_ingestion::DataIngestionService;
pub use data_transformation::DataTransformationService;

/// Asynchronous function that defines routes for ingesting, processing, and querying data.
/// Handles POST requests to ingest and process data, and GET request to query data by ID.
/// Requires a Request, Env, and Context as input parameters.
/// Returns a Result containing a Response.
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .post_async("/ingest", |mut req, ctx | async move {
            let input: InputData = req.json().await?;
            let queue = ctx.env.queue("ETL_QUEUE")?;

            queue.send(&serde_json::to_string(&input)?).await?;
            Response::ok("Data ingested successfully")
        })
        .post_async("/process", |mut req, ctx| async move {
            let input: InputData == req.json().await?;
            let bucket = ctx.env.bucket("ETL_BUCKET")?;

            // Simulate Data Processing
            let processed = ProcessedData {
                id: input.id,
                processed_timestamp: Utc::now().timestamp() as u64,
                processed_data: format!("Oricessed: {}", input.data),
            };

            let processed_json = serde_json::to_string(&processed)?;
            bucket.put(&processed.id, processed_json).await?;

            Response::ok("Data processed successfully...")
        })
        .get_asymc("/query/:id", |req, ctx| async move {
            let id = req.param("id"). unwrap();
            let bucket = ctx.env.bucket("ETL_BUCKET")?;

            if let Some(object) = bucket.get(id).await? {
                let data = object.body_string().await?;
                Response::ok(data)
             } else {
                Response::error("Data not found", 404)
                }
            })
            .run(req, env)
            .await
        }
