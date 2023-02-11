// // use actix_web::{
// //     cookie, get,
// //     http::{
// //         header::{CacheControl, CacheDirective},
// //         StatusCode,
// //     },
// //     middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
// // };
// // use serde::{Deserialize, Serialize};
// // use std::{
// //     net::Ipv4Addr,
// //     sync::{Arc, Mutex},
// //     time::Duration,
// //     vec, borrow::Cow,
// // };

// // #[derive(Debug)]
// // struct ServCounter {
// //     count: Mutex<i64>,
// // }

// // #[derive(Deserialize, Serialize)]
// // struct Something {
// //     value: i32,
// // }

// // #[get("/")]
// // async fn index(req: HttpRequest) -> impl Responder {
// //     println!("api call");
// //     HttpResponse::build(StatusCode::ACCEPTED)
// //         .insert_header(CacheControl(vec![
// //             CacheDirective::MaxAge(300u32),
// //             CacheDirective::Public,
// //         ]))
// //         .cookie(cookie::Cookie::new("zero", "0"))
// //         .cookie(cookie::Cookie::new("one", "1"))
// //         .cookie(cookie::Cookie::new("two", "2"))
// //         .cookie(
// //             cookie::Cookie::build("FINAL", "fANTASY")
// //                 .secure(true)
// //                 .http_only(true)
// //                 .path("/")
// //                 .same_site(cookie::SameSite::Strict)
// //                 .finish(),
// //         )
// //         .json(vec![
// //             Something { value: 0xedb },
// //             Something { value: 0xc0de },
// //         ])
// // }

// // #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// //     let counter = web::Data::new(ServCounter {
// //         count: Mutex::new(0xedb),
// //     });

// //     let a: Cow<'_, i32> = 1.into();

// //     HttpServer::new(move || {
// //         // println!("hjeeradfdaf");
// //         App::new()
// //             .wrap(middleware::Logger::default())
// //             .service(index)
// //             .app_data(counter.clone())
// //             .app_data(39usize)
// //     })
// //     .bind((Ipv4Addr::new(127, 0, 0, 1), 2023))?
// //     .keep_alive(Duration::from_secs(20))
// //     .run()
// //     .await
// // }

// // // use std::str::FromStr;

// // // use strum::{Display, EnumCount, EnumString, EnumVariantNames, VariantNames};

// // // #[derive(Debug, EnumString, EnumCount, EnumVariantNames, Clone)]
// // // // #[strum(serialize_all = "lowercase")]
// // // enum Login {
// // //     Google,
// // //     Legacy,
// // //     Facebook,

// // //     EmailPassword,
// // // }

// // // fn main() {
// // //     let choice = Login::Google;

// // //     // println!("{}", choice.to_string());

// // //     let l = Login::from_str("gooGLE").unwrap();

// // //     println!("{l:?} {}", Login::COUNT);
// // //     println!("{:?}", Login::VARIANTS);
// // // }

// use std::borrow::Cow;

// #[derive(Debug)]
// struct Something {
//     val: i32,
// }

// impl From<i32> for Something {
//     fn from(value: i32) -> Self {
//         Something { val: value }
//     }
// }

// impl From<&str> for Something {
//     fn from(value: &str) -> Self {
//         Something {
//             val: value.parse().unwrap(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// enum Exp {
//     One(i32),
//     Two(i128),
// }

// fn gimme_something(x: i32) ->  Cow<'static, Exp> {
//     if x % 2 == 0 {Exp::One(x)} else {Exp::Two(x as i128)}
// }

// fn main() {

// }

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("before sleep");
    sleep(Duration::from_secs(2)).await;

    println!("after sleep");
}
