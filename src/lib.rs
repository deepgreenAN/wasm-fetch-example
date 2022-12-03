use gloo_console::log;
use gloo_net::http::{Request, RequestMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const API_DOMAIN: &str = if cfg!(test) {
    "http://127.0.0.1:8080/api"
} else {
    "/api"
};
const CORS_MODE: RequestMode = if cfg!(test) {
    RequestMode::Cors
} else {
    RequestMode::SameOrigin
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Client {
    pub id: usize,
    pub name: String,
    pub location: String,
}

#[derive(thiserror::Error, Debug)]
pub enum FetchError {
    #[error(transparent)]
    GlooNetError(#[from] gloo_net::Error),

    #[error("Not Found Error")]
    NotFoundError,

    #[error("Already Exists Error")]
    AlreadyExistsError,
}

pub async fn get_client(id: usize) -> Result<Client, FetchError> {
    let response = Request::get(&format!("{}/client/{}", API_DOMAIN, id))
        .mode(CORS_MODE)
        .send()
        .await?;

    if response.ok() {
        let client = response.json::<Client>().await?;
        Ok(client)
    } else {
        Err(FetchError::NotFoundError)
    }
}

pub async fn post_client(client: Client) -> Result<(), FetchError> {
    let res = Request::post(&format!("{}/client", API_DOMAIN))
        .mode(CORS_MODE)
        .json(&client)?
        .send()
        .await?;

    if res.ok() {
        Ok(())
    } else {
        Err(FetchError::AlreadyExistsError)
    }
}

// JavaScriptに渡す関数
#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn post_and_get_client() {
    let client = Client {
        id: 10,
        name: "Smith".to_string(),
        location: "Los Angeles".to_string(),
    };

    post_client(client.clone()).await.unwrap();
    log!(format!("posted client:{:?}", client));

    let client = get_client(client.id).await.unwrap();
    log!(format!("given client:{:?}", client));
}

// テストするモジュール

#[cfg(all(test, target_arch = "wasm32"))]
mod test {
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_get_client() {
        let res_ok = super::get_client(0).await;
        assert!(res_ok.is_ok());

        let client = res_ok.unwrap();
        assert_eq!(
            client,
            super::Client {
                id: 0,
                name: "John".to_string(),
                location: "NewYork".to_string(),
            }
        );

        let res_err = super::get_client(1).await;
        assert!(res_err.is_err());
    }

    #[wasm_bindgen_test]
    async fn test_post_client() {
        let client_ok = super::Client {
            id: 1,
            name: "Ethan".to_string(),
            location: "Washington".to_string(),
        };

        let res_ok = super::post_client(client_ok).await;
        assert!(res_ok.is_ok());

        let client_err = super::Client {
            id: 0,
            name: "John".to_string(),
            location: "NewYork".to_string(),
        };

        let res_err = super::post_client(client_err).await;
        assert!(res_err.is_err())
    }
}
