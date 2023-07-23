use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod db;
mod routes;

// #[get("/api/meigen/{id}")]
// async fn test() -> impl Responder {
//     let conn = db

//     // HttpResponse::Ok().body("test")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_con = db::connect_db();
    HttpServer::new(|| {
        App::new()
            .configure(routes::test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}