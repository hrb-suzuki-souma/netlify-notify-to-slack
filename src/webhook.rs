use crate::error;
use failure::Error;
use reqwest::Url;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug)]
pub struct SlackWebhook {
    url: String,
}

impl SlackWebhook {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn notify(&self, deploy_url: String, log_url: String) -> Result<(), Error> {
        let body = &success_request_param_factory(deploy_url.to_owned(), log_url.to_owned());
        let url = Url::from_str(self.url.as_str())?;

        let client = reqwest::Client::new();

        client
            .post(url)
            .json(body)
            .send()
            .map_err(|_| error::CliError::WebhookRequestFail {
                url: self.url.clone(),
            })?;

        Ok(())
    }
}

fn success_request_param_factory(deploy_url: String, log_url: String) -> SlackWebhookRequestParam {
    let attachment = SlackRequestAtttachment {
        author_name: Some("Netlify".into()),
        color: Some("#76ff14".into()),
        title: Some("Visit the changes live".into()),
        title_link: Some(deploy_url.into()),
        footer: Some("Netlify".into()),
        footer_icon: Some("https://www.netlify.com/img/press/logos/logomark.png".into()),
        text: Some(format!("Or check out the [build log]({})", log_url).into()),
    };
    SlackWebhookRequestParam {
        text: Some("Successful deploy".to_string()),
        attachments: Some(vec![attachment]),
    }
}

// See the details on https://api.slack.com/docs/message-attachments
#[derive(Debug, Serialize)]
struct SlackWebhookRequestParam {
    text: Option<String>,
    attachments: Option<Vec<SlackRequestAtttachment>>,
}

#[derive(Debug, Serialize)]
struct SlackRequestAtttachment {
    color: Option<String>,
    author_name: Option<String>,
    title: Option<String>,
    title_link: Option<String>,
    footer: Option<String>,
    footer_icon: Option<String>,
    text: Option<String>,
}
