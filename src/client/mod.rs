use crate::error::Result;

pub mod circle;
pub mod genre;
pub mod product;
pub mod product_api;
pub mod search;

/// API client for DLsite.
#[derive(Default, Clone, Debug)]
pub struct DlsiteClient {
    client: reqwest::Client,
}

impl DlsiteClient {
    const BASE_URL: &'static str = "https://www.dlsite.com/maniax";
    pub async fn get(&self, path: &str) -> Result<String> {
        let url = format!("{}{}", Self::BASE_URL, path);
        let body = self.client.get(&url).send().await?.text().await?;
        Ok(body)
    }
    pub async fn get_raw(&self, url: &str) -> Result<String> {
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }
}
