//! Product data got using "api" method.
//!
//! See [`crate::product`] for more information.

pub mod interface;
#[cfg(test)]
mod test;

use crate::{error::Result, DlsiteClient, DlsiteError};

use self::interface::ProductApiContent;

/// Get product data using "api" method.
///
/// For more information, see [`crate::product_api`].
impl DlsiteClient {
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
    /// use dlsite::{DlsiteClient, product::Product};
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = DlsiteClient::default();
    ///     let product = client.get_product_api("RJ01014447").await.unwrap();
    ///     assert_eq!(product.creators.unwrap().voice_by.unwrap()[0].name, "佐倉綾音");
    /// }
    /// ```
    pub async fn get_product_api(&self, id: &str) -> Result<ProductApiContent> {
        let json = self
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
                    return Err(DlsiteError::ParseError("No product found".to_string()));
                };

                Ok(json)
            }
            Err(e) => Err(DlsiteError::ParseError(format!(
                "Failed to parse json: {}",
                e
            ))),
        }
    }
}
