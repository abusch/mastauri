use reqwest::Client;
use url::Url;

use crate::Status;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error deserializing JSON response: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Mastodon {
    client: Client,
    base: Url,
}

impl Mastodon {
    pub fn new(base: Url) -> Result<Self> {
        Ok(Self {
            client: Client::builder().build()?,
            base,
        })
    }

    pub async fn public_timeline(&self) -> Result<Vec<Status>> {
        let url = self.base.join("timelines/public")?;
        let res = self
            .client
            .get(url)
            .query(&[("limit", "10")])
            .send()
            .await?;
        let statuses: Vec<Status> = serde_json::from_str(&res.text().await?)?;
        Ok(statuses)
    }
}
