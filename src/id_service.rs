use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

// Define a structure for the response
#[derive(Serialize)]
struct IdResponse {
    id: String,
}

// Handler to generate a unique ID
async fn generate_id() -> impl Responder {
    // Generate a unique ID
    let unique_id = Uuid::new_v4();

    // Return as JSON
    HttpResponse::Ok().json(IdResponse {
        id: unique_id.to_string(),
    })
}

pub async fn start_id_service() -> std::io::Result<()> {
    println!("Starting ID Generator Service...");

    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            .route("/generate-id", web::get().to(generate_id))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


