// use actix_web::{get, web, App, HttpServer, Responder};
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// struct Info {
//     msg: Option<String>,
// }

// #[get("/{msg}")]
// async fn index(path: web::Path<Info>, q: web::Query<Info>, j: web::Form<Info>) -> impl Responder {
//     let p = path.into_inner().msg;
//     println!(
//         "path: {} \t query: {}",
//         p.unwrap(),
//         q.msg.clone().unwrap_or_default()
//     );
//     println!("json:  {:?}", j.into_inner().msg);

//     if true {
//         return "Hello server"
//     }

//     "hmmm"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 2023))?
//         .run()
//         .await
// }

use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test 1adjsflk jadklfj lasdfj klasdfj; kladjsf lkkaldjf  kldjaf kl;djf klads;jf klasdjf k;lasdjf kldjas;flj asd;sfjk daskl;jf ;kldajf kasdl;jf aklsdf; asdfkladshjfkljasdfjkl;sdjaf;lksdjfl; adsjkf sdl;ajf dkasl;jfikldajfkl;jsdklafjkldjas;fkjwepowaiojfawehjfiopasdhnikaflhjk;sdlfhiopadsh fiopasdhf iodah fpiohdpaf ihbadpsuifvbhc uiopadhbncvioahsdICFhpbnaisncv pioasdb hnf")));

    HttpResponse::Ok()
        .content_type("text/plain")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stream))
        .bind(("127.0.0.1", 2023))?
        .run()
        .await
}
