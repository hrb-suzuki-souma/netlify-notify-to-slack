#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod error;
mod webhook;

use failure::Error;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize)]
struct NetlifyDeployOutput {
    deploy_url: String,
    logs: String,
}

fn main() -> Result<(), Error> {
    let scan = std::io::stdin();
    let mut line = String::new();

    scan.lock().read_to_string(&mut line)?;

    let deploy_output: NetlifyDeployOutput = serde_json::from_str(line.as_str())
        .map_err(|_| error::CliError::InvalidJsonInput { json: line.clone() })?;

    let slack_webhook_url =
        std::env::var("SLACK_WEBHOOK_URL").map_err(|_| error::CliError::WebhookUrlNotfound)?;

    let client = webhook::SlackWebhook::new(slack_webhook_url.to_string());

    client.notify(deploy_output.deploy_url.clone(), deploy_output.logs.clone())?;

    Ok(())
}
