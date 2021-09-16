/*
    cargo run --example arrest_example_1
*/
use serde;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // gather args for client:
    let base_url = String::from("");
    let mut bearer = String::from("");
    // define client
    let mut client = arrest::Client::new(base_url.clone(), bearer.clone());
    client.set_headers();
    client.set_client();
    //
    let urls: Vec<String> = vec![
        String::from("http://httpbin.org/anything"),
        String::from("http://httpbin.org/anything"),
    ];
    // zero value devinfo
    let httpbin = HttpBinAnything::default();
    let res = client.arrest(urls, httpbin).await.unwrap();
    dbg!(res);
}

//http://httpbin.org/anything
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
