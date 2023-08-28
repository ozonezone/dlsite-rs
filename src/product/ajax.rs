use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::{interface::WorkType, DlsiteClient, DlsiteError, Result};

/// Data of a product from the AJAX API.
#[derive(Debug, Clone, Deserialize)]
pub struct ProductAjax {
    pub maker_id: String,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub dl_count: Option<i32>,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub review_count: Option<i32>,
    pub rate_average_2dp: Option<f32>,
    pub rate_count: Option<i32>,
    pub work_name: String,
    pub price: i32,
    #[serde(deserialize_with = "deserialize_work_type")]
    pub work_type: WorkType,
}

fn deserialize_work_type<'de, D>(deserializer: D) -> std::result::Result<WorkType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    WorkType::from_str(&s).map_err(serde::de::Error::custom)
}

impl DlsiteClient {
    /// Get the AJAX data of multiple products.
    #[tracing::instrument(err)]
    pub async fn get_products_ajax(
        &self,
        product_ids: Vec<&str>,
    ) -> Result<HashMap<String, ProductAjax>> {
        let path = format!("/product/info/ajax?product_id={}", product_ids.join(","));
        let ajax_json_str = self.get(&path).await?;

        let json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;

        Ok(json)
    }
    /// Get the AJAX data of a product.
    pub async fn get_product_ajax(&self, product_id: &str) -> Result<ProductAjax> {
        let path = format!("/product/info/ajax?product_id={}", product_id);
        let ajax_json_str = self.get(&path).await?;

        let mut json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;
        let product = json
            .remove(product_id)
            .ok_or_else(|| DlsiteError::ParseError("Failed to parse ajax json".to_string()))?;

        Ok(product)
    }
}
