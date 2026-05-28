use axum::Router;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    println!("cwd: {:?}", std::env::current_dir());

    let app = Router::new()
        .route_service("/openapi", ServeFile::new("docs/specs/openapi.yaml"))
        .nest_service("/swagger-ui", ServeDir::new("docs/swagger-ui/dist"))
        .route("/docs", get(render_swagger));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2004").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn render_swagger() -> Result<Html<&'static str>, (StatusCode, String)> {
    Ok(Html(
        r#"
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="/swagger-ui/swagger-ui.css" />
  <meta charset="utf-8"/>
  <title>Swagger UI</title>
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="/swagger-ui/swagger-ui-bundle.js"></script>
  <script>
    SwaggerUIBundle({
      url: '/openapi',
      dom_id: '#swagger-ui',
    });
  </script>
</body>
</html>
"#,
    ))
}
