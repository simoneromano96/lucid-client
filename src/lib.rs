/// LucidKV Client
use reqwest::{Client, Error, Response};
use serde::Serialize;
use url::Url;

pub struct LucidKVClient {
    pub http_client: Client,
    pub base_url: Url,
    jwt: Option<String>,
}

impl LucidKVClient {
    /// Create a new basic Lucid KV Client
    pub fn new(option_base_url: Option<String>) -> Self {
        let http_client = Client::new();
        let mut base_url: Url =
            Url::parse(&option_base_url.unwrap_or(String::from("http://127.0.0.1:7020")))
                .expect("Invalid input url");

        base_url = base_url.join("/api/kv/").unwrap();

        Self {
            http_client,
            base_url,
            jwt: None,
        }
    }

    /// Stores data into Lucid DB
    /// Will panic if key is not URI safe
    pub async fn store_data<T>(&self, key: String, data: T) -> Result<Response, Error>
    where
        T: Serialize,
    {
        let url: Url = self.base_url.join(&key).unwrap();

        self.http_client.put(url).json(&data).send().await
    }

    /// Gets data from Lucid DB
    /// Will panic if key is not URI safe
    pub async fn get_data(&self, key: String) -> Result<Response, Error> {
        let url: Url = self.base_url.join(&key).unwrap();
        self.http_client.get(url).send().await
    }

    /// Delete data from Lucid DB
    /// Will panic if key is not URI safe
    pub async fn delete_data(&self, key: String) -> Result<Response, Error> {
        let url: Url = self.base_url.join(&key).unwrap();
        self.http_client.delete(url).send().await
    }

    /// Sends HEAD request to Lucid DB, will return true if key is present
    /// Will panic if key is not URI safe
    pub async fn is_key_present(&self, key: String) -> bool {
        let url: Url = self.base_url.join(&key).unwrap();
        match self.http_client.head(url).send().await.unwrap() {
            res => res.status() == reqwest::StatusCode::OK,
            _ => false,
        }
    }
}
