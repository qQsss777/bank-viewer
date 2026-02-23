use axum::{
    body::Body,
    http::{HeaderValue, Method, Request, Response, StatusCode},
    middleware::Next,
};

pub async fn cors_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    if req.method() == Method::OPTIONS {
        return Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE,OPTIONS")
            .header("Access-Control-Max-Age", "86400")
            .body(axum::body::Body::empty())
            .unwrap();
    }
    let mut res = next.run(req).await;
    res.headers_mut().insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    res.headers_mut().insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));
    res.headers_mut().insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET,POST,PUT,DELETE,OPTIONS"),
    );
    res
}
