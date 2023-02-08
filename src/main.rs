use actix_web::{get, web, App, HttpServer, Responder};
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Clone)]
struct ServCounter {
    count: Arc<Mutex<i64>>,
}

#[get("/")]
async fn index(data: web::Data<ServCounter>) -> impl Responder {
    let mut d = data.count.lock().unwrap();

    *d += 1;

    let ans = format!("{}", *d);

    ans
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = ServCounter {
        count: Arc::new(Mutex::new(0x0)),
    };

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(counter.clone()))
    })
    .bind((Ipv4Addr::new(127, 0, 0, 1), 2023))?
    .keep_alive(Duration::from_secs(20))
    .run()
    .await
}
