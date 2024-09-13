/// # Structs:
/// A vareity of data types for any given session
/// 

use std::fs::File
use uuid::uuid;

static TMP_FILEPATH: FilePath = "/Users/nullzero/Documents/repos/opencti/cloudflare/workers-rs/cloudflare-etl-pipeline/src/tmp";

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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SessionID(Uuid);

pub struct SessionContext {
    sessions: HashMap<SessionID, Session>,
    session_id: Session,
    timestamp: DateTime<Utc>,
    created_at: DateTime<Utc>
}


impl SessionContext {
    pub fn new() -> Self {
       SessionContext {
            sessions: HashMap::new(),
            session_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            created_at: Utc::now(),
        }
    }

    pub fn create_session(&mut self) -> SessionID {
        let id = sessionID(Uuid::new_v4());
        self.sesions.insert(id.clone(), Session {
            uuid
             }
    }

    pub fn get_session_id(&self, id: &SessionID) -> Uuid {
        self.sessions.get(id)
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

#[derive(Deserialize)]
struct InputData {
    id: String,
    session_id: uuid,
    timetamp: u64,
    data: String,
}


#[derive(Deserialize)]
struct ProcessedData {
    id: String,
    session_id: Uuid::v4,
}
