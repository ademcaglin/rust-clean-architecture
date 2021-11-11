use crate::models::users::*;
use crate::handlers::users::UserRegisterCommand;
use crate::handlers::users::UserRegisterCommandResult;
use crate::handlers::Command;
use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use hyper::{header, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let user_repo = &PostgesUserRepository {};
    user_repo.register("adem".to_string(), "adem".to_string());
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

macro_rules! json {
    (body: $body:expr) => {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from($body))?
    };

    (status: $status:tt, body: $body:expr) => {
        Response::builder()
            .status($status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from($body))?
    };
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    let cmd = UserRegisterCommand {
        username: "adem2".to_string(),
        email: "adem@gmail.com".to_string(),
    };
    let a: UserRegisterCommandResult = cmd.handle().unwrap();
    let s = StatusCode::OK;
    Ok(json!(status: s, body: a.id.to_string()))
    
}

pub mod handlers;
pub mod models;
