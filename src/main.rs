use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use std::fs;
use std::process::Command;


#[post("/update")]
async fn update(params: web::Json<ReqParam>) -> impl Responder {
    let key = fs::read_to_string("../keyfile").expect("Couldn't read file").trim().to_string();
    if &key == &params.req_key {
       let output = Command::new("../updateV2rayConfig.sh")
            .arg(&params.new_port)
            .output()
            .expect("Unable to update port");
        HttpResponse::Ok().body(output.status.to_string())
    } else { HttpResponse::Ok().body("Error!")  }
}

#[derive(serde::Deserialize)]
struct ReqParam {
    new_port: String,
    req_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(update)
    })
        .bind(("127.0.0.1", 8067))?
        .run()
        .await
}