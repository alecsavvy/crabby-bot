mod slack;
use dotenv::dotenv;
use slack::Slack;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bearer_token = std::env::var("BOT_USER_OAUTH_TOKEN").expect("fuck");
    let slack = Slack {
        bearer_token,
        channel: "C029EV0P7EY".into(),
    };

    slack.send_message("test").await;
}
