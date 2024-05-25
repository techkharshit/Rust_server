use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::fs;
use std::io::Write;

async fn get_file(info: web::Path<String>) -> impl Responder {
    let path = format!("./{}", info);
    
    match fs::read_to_string(&path) {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
}

async fn put_file(info: web::Path<String>, payload: String) -> impl Responder {
    let path = format!("./{}", info);
    
    match fs::write(&path, payload) {
        Ok(_) => HttpResponse::Ok().body("File updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{file}", web::get().to(get_file))
            .route("/{file}", web::put().to(put_file))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
