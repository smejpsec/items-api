use actix_web::{App, HttpServer};
use handler::services::*;
use std::{env, net::Ipv4Addr};

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
