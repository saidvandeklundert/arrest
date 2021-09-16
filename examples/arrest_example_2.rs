/*
    cargo run --example arrest_example_2
*/
use serde;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // define client
    let mut client = arrest::Client::new(String::from(""), String::from(""));
    client.set_headers();
    client.set_client();

    let mut httpbin_second = HttpBinAnything::default();
    httpbin_second.update_queue(String::from("http://httpbin.org/anything"));
    httpbin_second.update_queue(String::from("http://httpbin.org/anything"));
    let res = httpbin_second.get_queue(client).await;
    dbg!(res);
}

impl HttpBinAnything {
    pub async fn get_queue(self, client: arrest::Client) -> Vec<HttpBinAnything> {
        let res = client.arrest(self.queue.clone(), self).await.unwrap();
        res
    }
    pub fn update_queue(&mut self, url: String) {
        self.queue.push(url);
    }
    pub fn flush_queue(&mut self) -> Vec<String> {
        let u: Vec<String> = self.queue.drain(0..).collect();
        return u;
    }
}
//http://httpbin.org/anything
#[derive(Debug, Deserialize, Serialize, Clone)]
struct HttpBinAnything {
    method: String,
    url: String,
    #[serde(skip)]
    queue: Vec<String>,
}

impl Default for HttpBinAnything {
    fn default() -> HttpBinAnything {
        HttpBinAnything {
            method: String::from(""),
            url: String::from(""),
            queue: vec![],
        }
    }
}
