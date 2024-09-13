/// Session Management
/// 
/// 
use chrono::{ DateTime, Utc, Now };
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{ Duration, Instant };
use tokio::sync::RwLock;
use uuid:UUid;
use serde::{ Serialize, Deserialize };

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SessionId(Uuid);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionData {
    session_id: SessionId,
    user_id: String,
    session_created: Datetime,
    session_finished: Datetime,
    session_error: Vec<String>,
    session_duration: Duration,
    session_accesesd: Vec<String>
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Session {
    uuid: uuid,
    timestamp: DateTime<Utc>,
    user_id: String,
    user_name: String,
    start_session: DateTime<Utc>,
    end_session: Option<DateTime<Utc>>,
    data: SessionData,
    error: Vec<String>
}

pub struct SessionStore {
    sessions: HashMap<SessionId, Session>,
    timeout: Duration,
}

impl SessionStore {
    pub fn new(timeout: Duration) -> Self {
        SessionStore {
            sessions: HashMap::new()
            timeout,
        }
    }

    pub fn create_session(&mut self, user_id: String) -> SessionID {
        let id = sessionID(Uuid::new_v4());
        let session = Session {
            id: session_id.clone(),
            data: SessionData { user_id }
            last_accessed: Instant::now(),
        };
        self.sessions.insert(session_id.clone(), session)
        session_id
        }
    }

    pub fn get_session(&mut self, id: &SessionID) -> Option<&SessionDate> {
        if let Some(session) = self.sessions.get_mut(id) {
            session.last_accessed = Instant::now();
            Some (&session.data)
        } else {
            None
        }
    }

    fn end_session_with_timestamp(&mut self) {
        self.end_session = Some(Utc::now());
        let formatted_time = self.end_session.unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        log::info!("Session ended at: {}", formatted_time);
    }

    pub fn remove_session(&mut self, id: &SessionId) {
        self.sessions.remove(id);
    }

    pub fn cleanup_expired_sessions(&mut self) {
        let now = Instant::now();
        self.sessions.retain(|_, session| {
            now.duration_since(session.last_accessed) < self.timeout
        });
    }

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

