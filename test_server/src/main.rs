use std::error::Error;

use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    msg: String,
}

#[get("/")]
async fn index(info: web::Query<Info>) -> Result<&'static str, actix_web::Error> {
    println!("msg: {}", info.msg);

    Ok("ok\n")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    HttpServer::new(move || App::new().service(index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await?;

    Ok(())
}
