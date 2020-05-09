use reqwest::{Client, Error, Response};
use serde::Serialize;
use url::Url;

pub struct LucidKVClient {
    http_client: Client,
    base_url: Url,
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

    /// Stores data into Lucid db
    pub async fn store_data<T>(&self, key: String, data: T) -> Result<Response, Error>
    where
        T: serde::Serialize,
    {
        let url: Url = self.base_url.join(&key).unwrap();

        self.http_client.put(url).json(&data).send().await
    }

    pub async fn get_data(&self, key: String) -> Result<Response, Error> {
        let url: Url = self.base_url.join(&key).unwrap();
        self.http_client.get(url).send().await
    }

    pub async fn delete_data(&self, key: String) -> Result<Response, Error> {
        let url: Url = self.base_url.join(&key).unwrap();
        self.http_client.delete(url).send().await
    }

    pub async fn is_key_present(&self, key: String) -> Result<Response, Error> {
        let url: Url = self.base_url.join(&key).unwrap();
        self.http_client.head(url).send().await
    }
}

#[cfg(test)]
mod tests {
    use crate::LucidKVClient;
    use url::Url;

    // use reqwest;
    #[test]
    fn new_client() {
        let lucid_client = LucidKVClient::new(None);

        assert_eq!(
            lucid_client.base_url,
            Url::parse("http://127.0.0.1:7020/api/kv/").unwrap()
        );
    }

    #[test]
    fn new_client_with_url() {
        let lucid_client = LucidKVClient::new(Some("http://1.2.3.4:7020".into()));

        assert_eq!(
            lucid_client.base_url,
            Url::parse("http://1.2.3.4:7020/api/kv/").unwrap()
        );
    }
}
