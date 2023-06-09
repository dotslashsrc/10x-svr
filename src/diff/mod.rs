use std::env;
use actix_web::{Responder};
use serde::{Deserialize, Serialize};
use crate::gpt::ChatGPT;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffPayload {
    content: String
}

pub async fn process(payload: String) -> Result<String, Box<dyn std::error::Error>> {
    let chatgpt = ChatGPT::new(
        env::var_os("GPT_API").unwrap().into_string().unwrap(),
        env::var_os("GPT_TOKEN").unwrap().into_string().unwrap(),
        env::var_os("GPT_DIFF_PROMPT").unwrap().into_string().unwrap()
    );

    let response = chatgpt.generate_response(payload).await?;

    Ok(response.to_string())
}

pub async fn http_handler(payload: String) -> impl Responder {
    match process(payload).await {
        Ok(response) => {
            // Return a success response
            response
        }
        Err(err) => {
            // Return an error response
            err.to_string()
        }
    }
}

