macro_rules! json {
    (body: $body:expr) => {
        hyper::Response::new()
            .with_header(hyper::header::ContentType::json())
            .with_body($body)
    };
    (status: $status:tt, body: $body:expr) => {
        hyper::Response::new()
            .with_header(hyper::header::ContentType::json())
            .with_status(hyper::StatusCode::$status)
            .with_body($body)
    };
}