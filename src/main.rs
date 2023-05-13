use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{env, net::Ipv4Addr};

#[derive(Debug, Serialize)]
struct Item {
    name: String,
    amount: i32,
}

#[derive(Debug, Deserialize)]
struct ItemQuery {
    name: String,
}

#[get("/api/items")]
async fn get_item(query: web::Query<ItemQuery>) -> impl Responder {
    // TODO: 在庫確認処理
    let name = (query.name).to_string();
    let amount = 30;

    let item = Item { name, amount };
    HttpResponse::Ok().json(item)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| App::new().service(get_item))
        .bind((Ipv4Addr::LOCALHOST, port))?
        .run()
        .await
}
