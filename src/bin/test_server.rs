#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::{
        extract::{Json, Path},
        http::StatusCode,
        routing::{get, post},
        Router,
    };
    use tower_http::cors::{Any, CorsLayer};
    use wasm_fetch_example::Client;

    async fn get_client_handler(Path(id): Path<usize>) -> (StatusCode, Json<Client>) {
        if id == 0 {
            let client = Client {
                id: 0,
                name: "John".to_string(),
                location: "NewYork".to_string(),
            };

            (StatusCode::OK, Json(client))
        } else {
            (StatusCode::NOT_FOUND, Json(Client::default()))
        }
    }

    async fn post_client_handler(Json(client): Json<Client>) -> StatusCode {
        if client.id == 0 {
            StatusCode::CONFLICT
        } else {
            StatusCode::OK
        }
    }

    // CORSの設定
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let test_app: Router<()> = Router::new()
        .route("/api/client", post(post_client_handler))
        .route("/api/client/:id", get(get_client_handler))
        .layer(cors_layer);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(test_app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("--features server を指定してください")
}
