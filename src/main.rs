use actix_web::{
    cookie, get,
    http::{header, StatusCode},
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug)]
struct ServCounter {
    count: Mutex<i64>,
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::build(StatusCode::ACCEPTED)
        .cookie(cookie::Cookie::new("sample", "value"))
        .body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(ServCounter {
        count: Mutex::new(0xedb),
    });

    HttpServer::new(move || {
        // println!("hjeeradfdaf");
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .app_data(counter.clone())
            .app_data(39usize)
    })
    .bind((Ipv4Addr::new(127, 0, 0, 1), 2023))?
    .keep_alive(Duration::from_secs(20))
    .run()
    .await
}
