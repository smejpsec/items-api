use crate::models::*;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/items")]
async fn get_item(query: web::Query<ItemQuery>) -> impl Responder {
    // TODO: 在庫確認処理
    let name = (query.name).to_string();
    let amount = 30;

    let item = Item { name, amount };
    HttpResponse::Ok().json(item)
}
