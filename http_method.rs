use axum::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct MyData {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = route("/", get(|| async { response::Json(MyData { message: "GET Request".to_string() }) }))
        .route("/post", post(|| async { response::Json(MyData { message: "POST Request".to_string() }) }))
        .route("/put", put(|| async { response::Json(MyData { message: "PUT Request".to_string() }) }))
        .route("/delete", delete(|| async { response::Json(MyData { message: "DELETE Request".to_string() }) }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
