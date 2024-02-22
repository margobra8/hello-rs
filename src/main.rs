use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use uuid::Uuid;

async fn index() -> impl Responder {
    HttpResponse::Found().header("Location", "/static/index.html").finish()
}

async fn resource() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "name": "Hello from a Rust API"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/healthz", web::get().to(|| HttpResponse::Ok()))
            .route("/v1/resource", web::get().to(resource))
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(fs::Files::new("/favicon.ico", "static/favicon.ico"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}