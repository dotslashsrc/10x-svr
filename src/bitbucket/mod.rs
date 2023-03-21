use actix_web::{Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestEvent {
    pullrequest: PullRequest
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    id: u64,
    title: String,
    url: String,
    author: String,
    description: String,
}

// The bitbucket webhook
pub async fn webhook(payload: String) -> impl Responder {
    let pr: PullRequestEvent = match serde_json::from_str(&payload) {
        Ok(pr) => pr,
        Err(e) => {
            eprintln!("Failed to deserialize payload: {:?}", e);
            return "Failed to deserialize payload".to_string();
        }
    };

    // Process the pull request data
    // For example, you could add a comment to the pull request, or trigger a build
    println!("Received Pull Request: {:?}", pr);

    // TODO: fetch diff from bitbucket api
    // TODO: call diff::process
    // TODO: return diff::process result

    // Return a success response
    "Webhook received".to_string()
}