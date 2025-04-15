use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{client::common::WorkType, error::Result, DlsiteError};

use super::ProductClient;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct TranslationBonusLangs {
    pub child_count: i32,
    pub price: i32,
    pub price_in_tax: i32,
    pub price_tax: i32,
    pub recipient_available_count: i32,
    pub recipient_max: i32,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TranslationBonusLangsTypes {
    Arr(Vec<()>),
    Map(HashMap<String, TranslationBonusLangs>),
}
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct TranslationInfo {
    pub lang: Option<String>,
    pub original_workno: Option<String>,
    pub parent_workno: Option<String>,
    pub production_trade_price_rate: i32,
    pub is_volunteer: bool,
    pub translation_bonus_langs: TranslationBonusLangsTypes,
    pub is_translation_bonus_child: bool,
    pub is_translation_agree: bool,
    pub is_original: bool,
    pub is_child: bool,
    pub is_parent: bool,
    pub child_worknos: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Rank {
    pub rank_date: String,
    pub rank: i32,
    pub category: String,
    pub term: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct CountDetail {
    pub count: i32,
    pub review_point: i32,
    pub ratio: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct DlCountItem {
    pub edition_type: String,
    pub lang: String,
    pub workno: String,
    pub display_order: i32,
    pub display_label: String,
    pub edition_id: i32,
    pub label: String,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub dl_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct VoicePack {
    pub parent_official_price: Option<i32>,
    pub child_price: i32,
    pub parent_price: i32,
    pub sum_point: i32,
    pub sum_price: i32,
    pub product_ids: Vec<String>,
    pub child_official_price: Option<()>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct SalesEndInfo {
    pub can_download: bool,
    pub not_download: bool,
    pub end_date: HashMap<String, String>,
    pub end_date_proto: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Bonus {
    pub end_date_str: String,
    pub dist_flg: String,
    pub description: Option<String>,
    pub title: String,
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct LimitedFreeTerms {
    pub workno: String,
    pub id: i32,
    pub start_date: String,
    pub original_workno: Option<String>,
    pub end_date: String,
}

/// Data of a product from the AJAX API.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct ProductAjax {
    pub is_downloadable_touch: Option<bool>,
    pub dl_count_total: Option<i32>,
    pub rate_average: Option<i32>,
    pub rate_average_star: Option<i32>,
    pub title_work_count: Option<i32>,
    pub discount_rate: Option<i32>,
    pub discount_campaign_id: Option<i32>,
    pub title_volumn: Option<i32>,
    pub discount_calc_type: Option<String>,
    pub bulkbuy_key: Option<String>,
    pub campaign_id: Option<String>,
    pub official_price_str: Option<String>,
    pub discount_end_date: Option<String>,
    pub discount_to: Option<String>,
    pub is_tartget: Option<String>,
    pub share_title: Option<String>,
    pub samples: Option<String>,
    pub product_id: Option<String>,
    pub maker_name: Option<String>,
    pub title_id: Option<String>,
    pub title_name: Option<String>,
    pub title_name_masked: Option<String>,

    pub site_id: String,
    pub site_id_touch: String,
    pub price_str: String,
    pub down_url: String,
    pub work_name_masked: String,
    pub work_image: String,
    pub regist_date: String,
    pub default_point_str: String,
    pub options: String,
    pub dlsiteplay_work: bool,
    pub is_discount: bool,
    pub is_pointup: bool,
    pub is_rental: bool,
    pub is_ana: bool,
    pub is_sale: bool,
    pub is_title_completed: bool,
    pub is_limit_work: bool,
    pub is_sold_out: bool,
    pub is_reserve_work: bool,
    pub is_reservable: bool,
    pub is_timesale: bool,
    pub is_free: bool,
    pub is_oly: bool,
    pub is_led: bool,
    pub is_noreduction: bool,
    pub is_wcc: bool,
    pub is_pack_work: bool,

    pub upgrade_min_price: i32,
    pub limit_stock: i32,
    pub timesale_stock: i32,
    pub official_price: i32,
    pub age_category: i32,
    pub affiliate_deny: i32,
    pub dl_format: i32,
    pub wishlist_count: i32,
    pub price_without_tax: i32,
    pub default_point_rate: i32,
    pub default_point: i32,
    pub on_sale: i32,

    pub currency_official_price: Option<HashMap<String, f64>>,
    pub locale_official_price: Option<HashMap<String, f64>>,
    pub book_type: Option<HashMap<String, Option<String>>>,
    pub locale_official_price_str: Option<HashMap<String, String>>,
    pub translators: Option<Vec<ProductAjax>>,
    pub sales_end_info: Option<SalesEndInfo>,
    pub voice_pack: Option<VoicePack>,
    pub dl_count_items: Option<Vec<DlCountItem>>,

    pub locale_price: HashMap<String, f64>,
    pub currency_price: HashMap<String, f64>,
    pub custom_genres: Vec<String>,
    pub locale_price_str: HashMap<String, String>,
    pub translation_info: TranslationInfo,
    pub rank: Vec<Rank>,
    pub rate_count_detail: Vec<CountDetail>,
    pub bonuses: Vec<Bonus>,
    pub limited_free_terms: Vec<LimitedFreeTerms>,
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

    // TODO: Investigate these fields
    pub product_point_rate: Option<Value>,
    pub discount_caption: Option<Value>,
    pub gift: Vec<Value>,
    pub work_rentals: Vec<Value>,
}

fn deserialize_work_type<'de, D>(deserializer: D) -> std::result::Result<WorkType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    WorkType::from_str(&s).map_err(serde::de::Error::custom)
}

impl<'a> ProductClient<'a> {
    /// Get the AJAX data of multiple products.
    #[tracing::instrument(err)]
    pub async fn get_products_ajax(
        &self,
        product_ids: Vec<&str>,
    ) -> Result<HashMap<String, ProductAjax>> {
        let path = format!("/product/info/ajax?product_id={}", product_ids.join(","));
        let ajax_json_str = self.c.get(&path).await?;

        let json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;

        Ok(json)
    }
    /// Get the AJAX data of a product.
    pub async fn get_product_ajax(&self, product_id: &str) -> Result<ProductAjax> {
        let path = format!("/product/info/ajax?product_id={}", product_id);
        let ajax_json_str = self.c.get(&path).await?;

        let mut json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;
        let product = json
            .remove(product_id)
            .ok_or_else(|| DlsiteError::Parse("Failed to parse ajax json".to_string()))?;

        Ok(product)
    }
}
