use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}