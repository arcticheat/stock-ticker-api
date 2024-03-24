use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/end-of-day/{symbol}")]
async fn get_end_of_day_data(path: web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok().body("hello endOfDay")
}

#[get("/api/intraday/{symbol}")]
async fn get_intraday_data() -> impl Responder {
    HttpResponse::Ok().body("hello intraday")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_end_of_day_data)
            .service(get_intraday_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}