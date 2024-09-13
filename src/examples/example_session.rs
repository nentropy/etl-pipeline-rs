// Example usage with an HTTP framework (e.g., actix-web)
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

struct AppState {
    session_store: Arc<RwLock<SessionStore>>,
}

async fn create_session(data: web::Data<AppState>) -> impl Responder {
    let mut store = data.session_store.write().await;
    let session_id = store.create_session("user123".to_string());
    HttpResponse::Ok().json(session_id.0)
}

async fn get_session_data(
    session_id: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut store = data.session_store.write().await;
    let session_id = SessionId(session_id.into_inner());
    if let Some(session_data) = store.get_session(&session_id) {
        HttpResponse::Ok().json(session_data)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session_store = Arc::new(RwLock::new(SessionStore::new(Duration::from_secs(3600))));
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                session_store: session_store.clone(),
            }))
            .route("/create_session", web::post().to(create_session))
            .route("/session/{session_id}", web::get().to(get_session_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}