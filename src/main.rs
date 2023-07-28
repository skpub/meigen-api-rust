use ::config::Config;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dbconfig::DBConfig;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use handlers::{add_user, get_users};

mod db;
mod models;
mod errors;
mod dbconfig;
mod handlers;

#[get("/api/meigen/{id}")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: DBConfig = config_.try_deserialize().unwrap();
    
    let pool = config.pg.create_pool(None,NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/users")
                .route(web::post().to(add_user))
                .route(web::get().to(get_users)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    server.await
}