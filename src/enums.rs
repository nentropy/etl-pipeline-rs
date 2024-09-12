use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize, Serializer};
use crate::to_do::enums::TaskStatus::{DONE, PENDING};
use syn_crabs::setup_logging;

setup_logging();

#[derive(Clone)]
pub enum PipelineStage {
    Extraction,
    Transformation,
    Loading,
    Validation,
}

#[derive(Debug)]
enum LogLevel {
    Info,
    Debug,
    Error,
}

fn log_message(level: LogLevel, message: &str) {
    match level {
        LogLevel::Info => log::info!("INFO: {}", message),
        LogLevel::Debug => log::debug!("DEBUG: {}", message),
        LogLevel::Error => log::error!("ERROR: {}", message),
    }
}

#[derive(Debug, Clone)]
enum SourceType {
    RelDB,
    VecDB,
    APISource,
}

#[derive(Debug, Clone)]
enum DataStatus {
    Raw,
    Processed,
    InProcess,
    Error
}

struct PipelineStage {
    Extraction,
    Transformation,
    Loading,
    Validation,
}


impl PipelineStage {
    match stage {
        PipelineStage::Extraction => log::info!("Extracting data..."),
        PipelineStage::Transformation => log::info!("Transforming data..."),
        PipelineStage::Loading => log::info!("Loading data..."),
        PipelineStage::Validation => log::info!("Validating data..."),
    }    
}

    pub fn hash_pw(password: String) -> Self {
        match validate_pw.as_str() {
            "INVALID" => INVALID,
            "VALID" => VALID,
            _ => panic!("input {} not supported", input_string)
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            DONE => write!(f, "DONE"),
            PENDING => write!(f, "PENDING")
        }
    }
}

