#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::{
        extract::{Json, Path, State},
        http::StatusCode,
        routing::{get, get_service, post},
        Router,
    };
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tower_http::services::{ServeDir, ServeFile};
    use wasm_fetch_example::Client;

    async fn get_client_handler(
        Path(id): Path<usize>,
        State(app_state): State<Arc<Mutex<HashMap<usize, Client>>>>,
    ) -> (StatusCode, Json<Client>) {
        match app_state.lock().unwrap().get(&id) {
            Some(client) => (StatusCode::OK, Json(client.clone())),
            None => (StatusCode::NOT_FOUND, Json(Client::default())),
        }
    }

    async fn post_client_handler(
        State(app_state): State<Arc<Mutex<HashMap<usize, Client>>>>,
        Json(client): Json<Client>,
    ) -> StatusCode {
        match app_state.lock().unwrap().entry(client.id) {
            Entry::Occupied(_) => StatusCode::CONFLICT,
            Entry::Vacant(v) => {
                v.insert(client);
                StatusCode::OK
            }
        }
    }

    let app_state: Arc<Mutex<HashMap<usize, Client>>> = Arc::new(Mutex::new(HashMap::new()));

    let api_app: Router<()> = Router::new()
        .route("/api/client", post(post_client_handler))
        .route("/api/client/:id", get(get_client_handler))
        .with_state(app_state);

    let app: Router<()> = Router::new()
        .route(
            "/",
            get_service(ServeFile::new("index.html"))
                .handle_error(|_| async move { StatusCode::NOT_FOUND }),
        )
        .nest_service(
            "/pkg",
            get_service(ServeDir::new("pkg"))
                .handle_error(|_| async move { StatusCode::NOT_FOUND }),
        )
        .merge(api_app);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("--features server を指定してください")
}
