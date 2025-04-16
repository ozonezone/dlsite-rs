//! Interfaces related to product api. For more information, see [`ProductApiClient`].

pub mod interface;
#[cfg(test)]
mod test;

use crate::{error::Result, DlsiteClient, DlsiteError};

use self::interface::ProductApiContent;

/// Client to retrieve DLsite product data using 'scraping' method
///
/// For difference about "scraping" and "api" method, see [`super::product::ProductClient`].
#[derive(Clone, Debug)]
pub struct ProductApiClient<'a> {
    pub(crate) c: &'a DlsiteClient,
}

impl<'a> ProductApiClient<'a> {
    /// Get product detail using api.
    ///
    /// # Arguments
    /// * `id` - Product ID.
    ///
    /// # Returns
    /// * [`ProductApiContent`] - Product details.
    ///
    /// # Note
    /// This api does not return dl count.
    ///
    /// # Example
    /// ```
    /// use dlsite::DlsiteClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = DlsiteClient::default();
    ///     let product = client.product_api().get("RJ01014447").await.unwrap();
    ///     assert_eq!(product.creators.unwrap().voice_by.unwrap()[0].name, "佐倉綾音");
    /// }
    /// ```
    pub async fn get(&self, id: &str) -> Result<ProductApiContent> {
        let json = self
            .c
            .get(&format!("/api/=/product.json?workno={}", id))
            .await?;
        let jd = &mut serde_json::Deserializer::from_str(&json);
        #[cfg(feature = "unknown-field-log")]
        let result: std::result::Result<Vec<ProductApiContent>, _> = serde_ignored::deserialize(
            jd,
            |path| {
                tracing::error!("Ignored path: '{}' for '{id}'. Please report this to https://github.com/ozonezone/dlsite-rs", path.to_string());
            },
        );
        #[cfg(not(feature = "unknown-field-log"))]
        let result: std::result::Result<Vec<ProductApiContent>, _> =
            serde_path_to_error::deserialize(jd);
        match result {
            Ok(result) => {
                let Some(json) = result.into_iter().next() else {
                    return Err(DlsiteError::Parse("No product found".to_string()));
                };

                Ok(json)
            }
            Err(e) => Err(DlsiteError::Parse(format!("Failed to parse json: {}", e))),
        }
    }
}
