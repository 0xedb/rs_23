// use actix_web::{
//     cookie, get,
//     http::{header, StatusCode},
//     middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
// };
// use std::{
//     net::Ipv4Addr,
//     sync::{Arc, Mutex},
//     time::Duration,
// };

// #[derive(Debug)]
// struct ServCounter {
//     count: Mutex<i64>,
// }

// #[get("/")]
// async fn index(req: HttpRequest) -> impl Responder {
//     HttpResponse::build(StatusCode::ACCEPTED)
//         .cookie(cookie::Cookie::new("sample", "value"))
//         .body("hello")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(ServCounter {
//         count: Mutex::new(0xedb),
//     });

//     HttpServer::new(move || {
//         // println!("hjeeradfdaf");
//         App::new()
//             .wrap(middleware::Logger::default())
//             .service(index)
//             .app_data(counter.clone())
//             .app_data(39usize)
//     })
//     .bind((Ipv4Addr::new(127, 0, 0, 1), 2023))?
//     .keep_alive(Duration::from_secs(20))
//     .run()
//     .await
// }

use std::str::FromStr;

use strum::{Display, EnumCount, EnumString, EnumVariantNames, VariantNames};

#[derive(Debug, EnumString, EnumCount, EnumVariantNames, Clone)]
// #[strum(serialize_all = "lowercase")]
enum Login {
    Google,
    Legacy,
    Facebook,

    EmailPassword,
}

fn main() {
    let choice = Login::Google;

    // println!("{}", choice.to_string());

    let l = Login::from_str("gooGLE").unwrap();

    println!("{l:?} {}", Login::COUNT);
    println!("{:?}", Login::VARIANTS);
}
