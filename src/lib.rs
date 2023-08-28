#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

use thiserror::Error;

pub mod circle;
pub mod genre;
pub mod interface;
pub mod product;
pub mod product_api;
pub mod search;
mod utils;

#[derive(Debug, Error)]
pub enum DlsiteError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("{0}")]
    ParseError(String),
    #[error("{0}")]
    ServerError(String),
}

pub(crate) type Result<T> = std::result::Result<T, DlsiteError>;

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
