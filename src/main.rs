use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use hyper::{header, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
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

    (status: $status:tt, body: $body:expr)  => {
        Response::builder()
            .status($status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from($body))?
    };
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    let s =StatusCode::OK;
    Ok(json!(status: s, body: "abc"))
}
