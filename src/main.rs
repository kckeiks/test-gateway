use axum::routing::get;
use axum::{Extension, Router};
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum_server::tls_rustls::RustlsConfig;
use hyper::Client;
use hyper::client::HttpConnector;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let bind_addr = "0.0.0.0:8081".parse().unwrap();
    let app = Router::new()
        .route("/", get(get_car_handler))
        .layer(Extension(client));
    let rustls_config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .unwrap();
    axum_server::bind_rustls(bind_addr, rustls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn get_car_handler(
    Extension(client): Extension<Client<HttpConnector>>,
) -> Response {
    client.get("http://google.com".parse().unwrap()).await.unwrap().into_response()
}