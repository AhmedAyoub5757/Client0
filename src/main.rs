use actix_web::{App, HttpServer, HttpResponse, web};
use bytes::Bytes;
use std::fs;

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("static/index.html"))
}

async fn upload_file(payload: Bytes) -> HttpResponse {
    // Store the uploaded file
    let file_path = "uploads/uploaded_file.xlsx";
    if let Err(err) = fs::write(file_path, &payload) {
        println!("Error storing file: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    // Return the path to the uploaded file
    HttpResponse::Ok().body(file_path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
