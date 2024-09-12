use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;
//use super::super::enums::TaskStatus; @TODO???

/// Specifiy Session
/// 
/// 

#[derive(Serialize)]
pub struct ETLPipelineBase {
        config: PipelineConfig,
        logging: LoggingConfig,
        error_handler: ErrorHandler,
        performance_metrics: PerformanceMetrics,
        data_source: DataSourceConfig,
        data_destination: DataDestinationConfig,
        transformation_rules: TransformationRules,
        metadata: Metadata,
    }

struct Session {
    uuid: uuid,
    timestamp: DateTime<Utc>,
    user_id: String,
    user_name: String,
    start_session: DateTime<Utc>,
    end_session: Option<DateTime<Utc>>,
    error: Vec<String>
}
impl Session {
    let new_session = Session
         // implemented in a textfile for now
        
        // Implement password verification logic here
        // For now, we'll just create a new session
        Ok(Session {
            uuid: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id: user.id.to_string(),
            user_name: user.username.clone(),
            start_session: Utc::now(),
            end_session: None,
            error: Vec::new(),
        })
    }

    fn end_session_with_timestamp(&mut self) {
        self.end_session = Some(Utc::now());
        let formatted_time = self.end_session.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        log::info!("Session ended at: {}", formatted_time);
    }
}


pub struct SessionContext {
    session_id: Session,
    timestamp: DateTime<Utc>,
    created_at: DateTime<Utc>
}
}


impl SessionContext {
    pub fn new() -> Self {
        Self {
            session_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            created_at: Utc::now(),
        }
    }

    pub fn get_session_id(&self) -> Uuid {
        self.session_id
    }
}

struct User {
    id: Uuid,
    username: String,
    password: PasswordHash
}

struct Session {
    uuid: Uuid,
    timestamp: DateTime<Utc>,
    user_id: String,
    user_name: String,
    start_session: DateTime<Utc>,
    end_session: Option<DateTime<Utc>>,
    error: Vec<String>
}

// Remove invalid trait declaration
// trait SessionID(let uuid::new_v4()) -> Result<(uuid,<Err(e)>>)

impl Session {
    pub fn start_session(user: &User, password: &str) -> Result<Session, String> {
        if password != hash_pw(password_)
        
        // Implement password verification logic here
        // For now, we'll just create a new session
        Ok(Session {
            uuid: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id: user.id.to_string(),
            user_name: user.username.clone(),
            start_session: Utc::now(),
            end_session: None,
            error: Vec::new(),
        })
    }

    fn end_session_with_timestamp(&mut self) {
        self.end_session = Some(Utc::now());
        let formatted_time = self.end_session.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        log::info!("Session ended at: {}", formatted_time);
    }
}