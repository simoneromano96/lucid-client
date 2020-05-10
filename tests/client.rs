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

#[tokio::test]
async fn get_data() {
    let lucid_client = LucidKVClient::new(None);

    let data = DemoData {
        demo: "Something".into(),
    };

    let res_create = lucid_client.store_data("something".into(), data).await;
    assert_eq!(res_create.unwrap().status(), reqwest::StatusCode::OK);

    let res = lucid_client.get_data("something".into()).await.unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn delete_data() {
    let lucid_client = LucidKVClient::new(None);

    let data = DemoData {
        demo: "Something".into(),
    };

    let res_create = lucid_client.store_data("something".into(), data).await;
    assert_eq!(res_create.unwrap().status(), reqwest::StatusCode::OK);

    let res = lucid_client.delete_data("something".into()).await.unwrap();
    assert_eq!(res.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn is_key_present() {
    let lucid_client = LucidKVClient::new(None);

    let res = lucid_client.is_key_present("not-present".into()).await;
    assert_eq!(res, false);

    let data = DemoData {
        demo: "Something".into(),
    };

    let res = lucid_client.store_data("present".into(), data).await;
    assert_eq!(res.unwrap().status(), reqwest::StatusCode::OK);

    let res = lucid_client.is_key_present("present".into()).await;
    assert_eq!(res, true);
}
