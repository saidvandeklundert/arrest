/*
    cargo run --example arrest_example_1
*/
use serde;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let mut client = arrest::Client::new();
    client.set_client(6, false);
    let urls: Vec<String> = vec![
        String::from("http://httpbin.org/anything"),
        String::from("http://httpbin.org/anything"),
    ];
    let httpbin = HttpBinAnything::default();
    let res = client.arrest(urls, httpbin).await.unwrap();
    dbg!(res);
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HttpBinAnything {
    method: String,
    url: String,
}

impl Default for HttpBinAnything {
    fn default() -> HttpBinAnything {
        HttpBinAnything {
            method: String::from(""),
            url: String::from(""),
        }
    }
}
