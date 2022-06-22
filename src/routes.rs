use actix_web::{get, web, HttpServer, Responder, Error, HttpResponse};

use crate::controllers::{wallet::{create_nft, get_balance, get_address, create_wallet}};
use crate::views::index::{render_index};


#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    render_index().await
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    // create_nft().await.map_err(|err| println!("{:?}", err));
    format!("Hello {name}!")
}

#[get("/balance")]
pub async fn balance() -> impl Responder {
    get_balance()
        .await
        .unwrap_or_else(|err| format!("err {err}!"))
}

#[get("/address")]
pub async fn address(name: web::Path<String>) -> impl Responder {
    get_address()
        .await
        .unwrap_or_else(|err| format!("err {err}!"))
}

#[get("/create")]
pub async fn create(name: web::Path<String>) -> impl Responder {
    create_wallet()
        .await
        .unwrap_or_else(|err| format!("err {err}!"))
}
