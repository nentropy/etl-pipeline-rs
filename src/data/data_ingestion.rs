/// Data Ingestion Model
/// - Kafka Compatible Redpanda Client Sending and Receiving Messages
/// 
/// Features:
/// - Retrieves Data
/// - Normalizes Data
/// 
/// Returns: Processed JSON
/// 
use rfkafka::config::ClientConfig;
use rfkafka::producer::{ FutureProducer, FutureRecord };
use rfkafka::consumer::{ StreamConsumer, Consumer };
use rfkafka::message::Message;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use anyhow::Result;

#[derive(Serialize, Deserialize)]
struct DataEvent {
    session_id: String,
    timestamp: i64,
    data: String,
}

pub struct DataIngestionService {
    producer: FutureProducer,
    consumer: StreamConsumer,
}

impl DataIngestionService {
    pub fn new(brokers:&str,  topic:&str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
    
        let consumer: StreamConsumer = ClientConfig::new() 
            .set("group_id", "etl_group")
            .set("bootstrap.servers", brokers)
            .set("enable.auto.commit", "true")
            .create()?;

        consumer.subscribe(&[topic])?;
        Ok(Self { producer, consumer });
    }
}

pub async fn ingest_data(&self, data: String) -> Result<()> {
    let event = DataEvent {
        session_id:Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        data,
    };


    let payload = serde_json::to_string(&event)?;
    let record = FutureRecord::to("input_topic"
        .payload(&payload)
        .key(&event.session_id));

    self.producer.send(record, std::time::Duratrion::from_secs(0)).await?;
    Ok(())
}
pub async fn consume_data(&self) -> Result<(DataEvent), Err<()>> {
    let message = self.consumer.recv().await?;
    let payload = message.payload_view::<str>()   
        .expect("Error while deserializing the payloa")
        .unwrap();

    let event: DataEvent = serde_json::from_string(payload)?;
    Ok(event)
    
}


// @TODO



