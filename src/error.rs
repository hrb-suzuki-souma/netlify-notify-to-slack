use failure::Fail;

#[derive(Debug, Fail)]
pub enum CliError {
    #[fail(display = "Invalid json input: {}", json)]
    InvalidJsonInput { json: String },
    #[fail(display = "Webhook url of slack must be set to $SLACK_WEBHOOK_URL")]
    WebhookUrlNotfound,
    #[fail(display = "Request failed to {}", url)]
    WebhookRequestFail { url: String },
}
