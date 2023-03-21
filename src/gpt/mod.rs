use std::borrow::Borrow;

use reqwest::{Client, Error};
use serde_json::json;

pub struct ChatGPT {
    endpoint: String,
    token: String,
    diff_prompt: String
}

impl ChatGPT {
    pub fn new(endpoint: String, token: String, diff_prompt: String) -> Self {
        ChatGPT {
            endpoint: endpoint,
            token: token,
            diff_prompt: diff_prompt
        }
    }

    pub async fn generate_response(&self, prompt: String) -> Result<String, Error> {
        let client = Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", self.token).parse().unwrap());

        let value = &json!({
            "model": "gpt-3.5-turbo",
            "messages": [{
                "role": "user",
                "content": self.diff_prompt.to_string() + "\n\n" + prompt.borrow()
            }]
        });
        
        let response = client.post(&self.endpoint)
            .headers(headers)
            .json(value)
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;
        let response_text = response_json["choices"][0]["message"]["content"].as_str().unwrap();

        Ok(response_text.to_string())
    }
}
