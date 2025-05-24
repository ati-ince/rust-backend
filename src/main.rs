use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn hello() -> impl Responder {
    let message = Message {
        content: "Hello from Rust Backend!".to_string(),
    };
    HttpResponse::Ok().json(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://localhost:8000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
