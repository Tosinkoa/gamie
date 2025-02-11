use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Health Check OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting backend server on port 8000");
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
} 