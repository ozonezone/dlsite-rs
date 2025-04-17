use crate::error::Result;

pub mod circle;
pub mod product;
pub mod product_api;
pub mod search;

/// API client for DLsite.
#[derive(Clone, Debug)]
pub struct DlsiteClient {
    client: reqwest::Client,
    base_url: String,
}

impl Default for DlsiteClient {
    fn default() -> Self {
        Self::new("https://www.dlsite.com/maniax")
    }
}

impl DlsiteClient {
    /// Create a new DLsite client with a custom base URL.
    ///
    /// Typical base URL is `https://www.dlsite.com/maniax` and you should be able to access any
    /// products using this URL, so usually you don't use this method and just use the default.
    pub fn new(base_url: &str) -> Self {
        let client = reqwest::Client::default();
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    /// Convenient method to make a http GET request using the client.
    pub async fn get(&self, path: &str) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        let body = self.client.get(&url).send().await?.text().await?;
        Ok(body)
    }

    /// Similar to `get`, but this method does not prepend the base URL.
    pub async fn get_raw(&self, url: &str) -> Result<String> {
        let body = self.client.get(url).send().await?.text().await?;
        Ok(body)
    }
}

/// These methods return a “sub-client”.
/// The sub-client has a DlsiteClient reference inside and has implementations of fetch and parse focused on certain purposes.
impl DlsiteClient {
    /// Get a client to fetch product info using 'scraping' method. For more information, see [`product::ProductClient`].
    pub fn product(&self) -> product::ProductClient {
        product::ProductClient { c: self }
    }

    /// Get a client to fetch product info using 'api' method. For more information, see
    /// [`product_api::ProductApiClient`].
    pub fn product_api(&self) -> product_api::ProductApiClient {
        product_api::ProductApiClient { c: self }
    }

    /// Get a client to fetch circle info. For more information, see [`circle::CircleClient`].
    pub fn circle(&self) -> circle::CircleClient {
        circle::CircleClient { c: self }
    }

    /// Get a client to search things. For more information, see [`search::SearchClient`].
    pub fn search(&self) -> search::SearchClient {
        search::SearchClient { c: self }
    }
}
