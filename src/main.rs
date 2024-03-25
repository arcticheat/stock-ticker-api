use actix_web::{get, web, http, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Deserialize;
use std::env;
use reqwest::{Result, blocking};


const MARKET_STACK_URL: &str = "https://api.marketstack.com/";
const EOD_PATH: &str = "eod";
const INTRADAY_PATH: &str = "intraday";

#[derive(Deserialize, Debug)]
struct Symbols {
    symbols: String
}

#[get("/api/end-of-day")]
async fn get_end_of_day_data(symbols: web::Query<Symbols>) -> impl Responder {
     let res = reqwest::blocking::get(format!("{MARKET_STACK_URL}/{EOD_PATH}?symbols={}", symbols.symbols))?;
     let data = res.status();
     
     HttpResponse::Ok().body(res)
    //HttpResponse::Ok().body(format!("hello {}", symbols.symbols))
}

// #[get("/api/intraday/{symbol}")]
// async fn get_intraday_data(path: web::Path<(String,)>) -> impl Responder {
//     HttpResponse::Ok().body("hello intraday")
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = "API_KEY";
    match env::var(key) {
        Ok(val) => println!("{key}: found"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }

    HttpServer::new(|| {
        let cors = Cors::default()
        .allowed_origin("http://localhost:4200")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);
        App::new()
            .wrap(cors)
            .service(get_end_of_day_data)
            // .service(get_intraday_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

