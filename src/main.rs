use actix_web::{get, App, HttpServer, Responder};

 
    "hello world"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:2023")?
        .run()
        .await
}


// try service config example