mod interface;
#[cfg(test)]
mod test;

use crate::{DlsiteClient, DlsiteError, Result};

use self::interface::ProductApiContent;

impl DlsiteClient {
    /// Get product detail using api.
    ///
    /// # Arguments
    /// * `id` - Product ID.
    ///
    /// # Returns
    /// * `ProductApiContent` - Product details.
    ///
    /// # Note
    /// This api does not return dl count.
    /// And because of confusing specification of api, serde::Value is used in some place.
    /// Instead of this you also can use `DlsiteClient.get_product` which scrapes html.
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
