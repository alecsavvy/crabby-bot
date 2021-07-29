use serde_json::json;

pub struct Slack {
    pub bearer_token: String,
    pub channel: String,
}

impl Slack {
    pub async fn send_message(&self, msg: &str) {
        let client = reqwest::Client::new();
        client
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(self.bearer_token.clone())
            .json(&json!({
                "channel": self.channel.clone(),
                "text": msg,
            }))
            .send()
            .await
            .unwrap();
    }
}
