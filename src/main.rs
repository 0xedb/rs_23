// use actix_web::{
//     cookie, get,
//     http::{
//         header::{CacheControl, CacheDirective},
//         StatusCode,
//     },
//     middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
// };
// use serde::{Deserialize, Serialize};
// use std::{
//     net::Ipv4Addr,
//     sync::{Arc, Mutex},
//     time::Duration,
//     vec, borrow::Cow,
// };

// #[derive(Debug)]
// struct ServCounter {
//     count: Mutex<i64>,
// }

// #[derive(Deserialize, Serialize)]
// struct Something {
//     value: i32,
// }

// #[get("/")]
// async fn index(req: HttpRequest) -> impl Responder {
//     println!("api call");
//     HttpResponse::build(StatusCode::ACCEPTED)
//         .insert_header(CacheControl(vec![
//             CacheDirective::MaxAge(300u32),
//             CacheDirective::Public,
//         ]))
//         .cookie(cookie::Cookie::new("zero", "0"))
//         .cookie(cookie::Cookie::new("one", "1"))
//         .cookie(cookie::Cookie::new("two", "2"))
//         .cookie(
//             cookie::Cookie::build("FINAL", "fANTASY")
//                 .secure(true)
//                 .http_only(true)
//                 .path("/")
//                 .same_site(cookie::SameSite::Strict)
//                 .finish(),
//         )
//         .json(vec![
//             Something { value: 0xedb },
//             Something { value: 0xc0de },
//         ])
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(ServCounter {
//         count: Mutex::new(0xedb),
//     });

//     let a: Cow<'_, i32> = 1.into();

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

// // use std::str::FromStr;

// // use strum::{Display, EnumCount, EnumString, EnumVariantNames, VariantNames};

// // #[derive(Debug, EnumString, EnumCount, EnumVariantNames, Clone)]
// // // #[strum(serialize_all = "lowercase")]
// // enum Login {
// //     Google,
// //     Legacy,
// //     Facebook,

// //     EmailPassword,
// // }

// // fn main() {
// //     let choice = Login::Google;

// //     // println!("{}", choice.to_string());

// //     let l = Login::from_str("gooGLE").unwrap();

// //     println!("{l:?} {}", Login::COUNT);
// //     println!("{:?}", Login::VARIANTS);
// // }

#[derive(Debug)]
struct Something {
    val: i32,
}

impl From<i32> for Something {
    fn from(value: i32) -> Self {
        Something { val: value }
    }
}

impl From<&str> for Something {
    fn from(value: &str) -> Self {
        Something {
            val: value.parse().unwrap(),
        }
    }
}

use std::convert::identity;

fn main() {
    let p: String = "300".into();
    let q: i128 = 0xedb.into();

    let a: Something = 0xedb.into();
    println!("{a:?}");

    // let ans = if true {
    //     Something { val: 30 }
    // } else {
    //     identity
    // };

    // let items = &[identity, Something{val: 0xe}];

    fn aa(x: i32) -> i32 {
        x + 1
    };
    let mut a = aa;

    if !true {
        println!("first {}", a(10))
    } else {
        println!("second {}", identity(10))
    }

    let ans = aa(20);

    let a = [1, 2, 3, 4].map(identity).into_iter().collect::<Vec<_>>();

    println!("{a:?}");
}
