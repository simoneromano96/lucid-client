use lucid_client::LucidKVClient;
use serde_derive::{Deserialize, Serialize};
use tokio;
use url::Url;

#[derive(Serialize)]
struct DemoData {
    demo: String,
}

// use reqwest;
#[tokio::test]
async fn new_client() {
    let lucid_client = LucidKVClient::new(None);

    assert_eq!(
        lucid_client.base_url,
        Url::parse("http://127.0.0.1:7020/api/kv/").unwrap()
    );
}

#[tokio::test]
async fn new_client_with_url() {
    let lucid_client = LucidKVClient::new(Some("http://1.2.3.4:7020".into()));

    assert_eq!(
        lucid_client.base_url,
        Url::parse("http://1.2.3.4:7020/api/kv/").unwrap()
    );
}

#[tokio::test]
async fn store_data() {
    let lucid_client = LucidKVClient::new(None);

    let data = DemoData {
        demo: "Something".into(),
    };

    let res = lucid_client.store_data("something".into(), data).await;
    assert_eq!(res.unwrap().status(), reqwest::StatusCode::OK);
}
