use crate::header::HeaderMap;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::header;
use serde;
use std::time::Duration;
use tokio::sync::mpsc;

/// REST client to make HTTP GET requests.
#[derive(Debug, Clone)]
pub struct Client {
    base_url: Option<String>,
    bearer: Option<String>,
    headers: HeaderMap,
    client: reqwest::Client,
}
/*
Use Client::new as a constructor.
After constructing the client, set the headers and the inn-client (reqwest).

Use client.api_call(&str) to make asynchronous API calls.
*/
impl Client {
    pub fn new() -> Self {
        Self {
            base_url: None,
            bearer: None,
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }
    pub fn set_bearer(&mut self, bearer: String) {
        self.bearer = Some(bearer);
    }

    pub fn set_headers(&mut self) {
        let mut headers = header::HeaderMap::new();
        let bearer = &self.bearer.clone().unwrap();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(bearer).unwrap(),
        );
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/json"),
        );
        self.headers = headers
    }

    pub fn set_client(&mut self, time_out: u64, accept_invalid_cert: bool) {
        let client: reqwest::Client = reqwest::Client::builder()
            .default_headers(self.headers.clone())
            .danger_accept_invalid_certs(accept_invalid_cert)
            .timeout(Duration::from_secs(time_out))
            .build()
            .unwrap();
        self.client = client
    }

    // Get target URL
    pub async fn api_call(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let res = self.client.get(url).send().await;
        return res;
    }

    // Make several asynchronous get requests for target URL:
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
            match body {
                Ok(text) => {
                    api_call_results.push(text);
                }
                Err(err) => println!("error making API call {}", err),
            }
        }
        return api_call_results;
    }

    // Make several asynchronous get requests for target URL:
    // fn generic<T>(_s: SGen<T>) {}
    pub async fn arrest<'a, T: serde::de::DeserializeOwned>(
        self,
        paths: Vec<String>,
        struct_response: T,
    ) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
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
            let response = api_call_result?;
            println!("reqwest result: {:?}", response.status());
            let text = response.text().await?;
            api_call_results.push(text.to_string());
        }
        // build the serialized data and return it:
        let resulting_serialized = self.deserialize(api_call_results.clone(), struct_response)?;
        return Ok(resulting_serialized);
    }

    // Deserialize the struct
    pub fn deserialize<'a, T: serde::de::DeserializeOwned>(
        self,
        api_responses: Vec<String>,
        struct_response: T,
    ) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut vec_of_structs: Vec<T> = Vec::new();
        let api_text_results = api_responses.iter();
        for val in api_text_results {
            let owner = val.to_string();
            let serde_result = serde_json::from_str(&owner);
            match serde_result {
                Ok(json_str) => {
                    let json_data: T = json_str;
                    vec_of_structs.push(json_data);
                }
                Err(err) => println!("error deserializing for struct {}", err), // need to come up with a logger??
            }
        }
        return Ok(vec_of_structs);
    }
}

#[async_trait]
pub trait Arrest {
    async fn run(&self) {
        println!("Good morning.")
    }
}
pub fn function() {
    println!("called `my_mod::function()`");
}
