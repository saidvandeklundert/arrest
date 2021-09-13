use crate::header::HeaderMap;
use reqwest::header;
use serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Duration;
use tokio::sync::mpsc;

/// REST client to make HTTP GET requests.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    bearer: String,
    headers: HeaderMap,
    client: reqwest::Client,
}
/*
Use Client::new as a constructor.
After constructing the client, set the headers and the inn-client (reqwest).

Use client.api_call(&str) to make asynchronous API calls.
*/
impl Client {
    pub fn new(base_url: String, bearer: String) -> Self {
        Self {
            base_url: base_url,
            bearer: bearer,
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }
    pub fn set_headers(&mut self) {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&self.bearer).unwrap(),
        );
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/json"),
        );
        self.headers = headers
    }

    pub fn set_client(&mut self) {
        let client: reqwest::Client = reqwest::Client::builder()
            .default_headers(self.headers.clone())
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(9))
            .build()
            .unwrap();
        self.client = client
    }
    pub async fn api_call(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let res = self.client.get(url).send().await;
        return res;
    }
    pub async fn api_call_vec(self, paths: Vec<String>) -> Vec<String> {
        let (tx, mut rx) = mpsc::channel(32);
        for path in paths {
            let tx = tx.clone();
            let aself = self.clone();
            tokio::spawn(async move {
                tx.send(aself.api_call(&path).await).await;
            });
        }
        drop(tx);
        let mut api_call_results: Vec<String> = Vec::new();
        // Read from all the channels:
        while let Some(api_call_result) = rx.recv().await {
            let response = api_call_result.unwrap();
            println!("reqwest result: {:?}", response.status());
            let body = response.text().await;
            let text: String = body.unwrap();
            api_call_results.push(text);
        }
        return api_call_results;
    }
}
