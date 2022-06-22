use actix_web::{web, HttpServer, Responder, App};

mod controllers;
mod views;
mod routes;

use routes::{address, balance, create, greet, index};

use dotenv::dotenv;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // TODO: - load DID it here - if not exists, create a new one and store it.
    // create_did().await;
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(balance)
            .service(create)
            .service(address)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
