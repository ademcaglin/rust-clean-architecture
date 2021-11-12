use domain::users::*;
use common::cqrs::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

macro_rules! create_query  {
    ($qt: ty, $rt: ty) => {
        {
           let user_query = <$qt>::default();
           let r: $rt = user_query.handle().unwrap();
           HttpResponse::Ok().json(r) 
        }
    };
}

#[post("/users")]
async fn register_user(cmd: web::Json<UserRegisterCommand>) -> impl Responder {
    let r = cmd.handle().unwrap();
    HttpResponse::Ok().json(r)
}

#[get("/users")]
async fn get_users() -> impl Responder {
   create_query!(UsersPageRequest, UsersPageResult)
}
/*async fn get_users() -> impl Responder {
    let user_query = UsersPageRequest::default();
    let r: UsersPageResult = user_query.handle().unwrap();
    HttpResponse::Ok().json(r)
}*/

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(register_user)
            .service(get_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub mod domain;
pub mod infra;
pub mod common;
