use std::collections::HashMap;

use either::Either;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DefaultOnError};

use crate::{
    interface::{AgeCategory, FileType, WorkCategory, WorkType},
    DlsiteClient, DlsiteError, Result,
};

pub type ProductApiResult = Vec<ProductApiContent>;

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct ProductApiContent {
    pub age_category: AgeCategory,
    pub age_category_string: String,
    pub anime: Option<String>,
    pub auto_play: Option<String>,
    pub bgm: Option<String>,
    pub bgm_mode: Option<String>,
    pub books_id: Option<String>,
    pub brand_id: Option<String>,
    pub circle_id: Option<String>,
    pub coupling: Vec<String>,
    pub cpu: Option<String>,
    pub default_point: i64,
    pub directed_by: Option<String>,
    pub directx: Option<String>,
    pub discount: Option<Value>,
    pub dist_flag: i64,
    pub dl_format: i64,
    pub etc: Option<String>,
    pub file_date: Option<String>,
    pub file_size: Option<String>,
    pub file_type: FileType,
    pub file_type_string: String,
    pub file_type_special: Option<String>,
    pub gallery_mode: Option<String>,
    pub hdd: Option<String>,
    pub h_scene_mode: Option<String>,
    pub intro: Option<String>,
    pub intro_s: String,
    pub label_id: Option<String>,
    pub label_name: Option<String>,
    pub machine: Option<String>,
    // pub machine_string_list: HashMap<String, Option<String>> | Vec<>,
    pub machine_string_list: Value,
    pub memory: Option<String>,
    pub message_skip: Option<String>,
    pub mini_resolution: Option<String>,
    pub modify_flg: Option<i64>,
    pub music_by: Option<String>,
    pub on_sale: i64,
    pub options: String,
    pub original_illust: Option<String>,
    pub other: Option<String>,
    pub others_by: Option<String>,
    pub pages: Option<String>,
    pub page_number: Option<String>,
    pub product_point: Option<i64>,
    pub product_point_end_date: Option<i64>,
    pub point: i64,
    pub price: i64,
    pub price_without_tax: i64,
    pub price_en: f64,
    pub price_eur: f64,
    pub production_workno: Option<String>,
    pub publisher_workno: Option<String>,
    pub regist_date: Option<String>,
    pub regular_price: Option<i64>,
    pub scenario_by: Option<String>,
    pub screen_mode: Option<String>,
    pub series_id: Option<String>,
    pub series_name: Option<String>,
    pub sex_category: i64,
    pub sofrin_app_no: Option<String>,
    pub vocal_track: Option<String>,
    pub voice: Option<String>,
    pub voice_by: Option<String>,
    pub vram: Option<String>,
    pub workno: String,
    pub work_name: String,
    pub work_name_kana: String,
    pub work_type: WorkType,
    pub work_type_string: String,
    pub work_type_special: Option<String>,
    pub work_attributes: String,
    pub product_id: String,
    pub base_product_id: String,
    pub maker_id: String,
    pub maker_name: String,
    pub maker_name_en: Option<String>,
    pub alt_name: String,
    pub product_name: String,
    pub site_id: String,
    pub site_id_touch: String,
    pub is_ana: bool,
    pub work_category: WorkCategory,
    pub platform: Vec<String>,
    pub is_pc_work: bool,
    pub is_smartphone_work: bool,
    pub is_android_only_work: bool,
    pub is_dlplaybox_only_work: bool,
    pub is_almight_work: bool,
    pub is_dlsiteplay_work: bool,
    pub is_dlsiteplay_only_work: bool,
    pub work_parts: Vec<String>,
    pub introductions: Option<String>,
    pub sales_price: Option<i64>,
    pub image_main: File,
    pub image_thum: File,
    pub image_thum_mini: File,
    #[serde(with = "either::serde_untagged")]
    pub image_thum_touch: Either<Vec<File>, File>,
    pub image_thum_mini_touch: Vec<Image>,
    pub image_mini: Image,
    pub image_samples: Option<Vec<File>>,
    pub image_thumb: String,
    pub image_thumb_touch: String,
    /// Files contained
    pub contents: Vec<Content>,
    /// Files contained (for touch device)
    pub contents_touch: Option<Vec<Content>>,
    pub is_split_content: bool,
    pub content_count: i64,
    pub content_count_touch: i64,
    pub contents_file_size: i64,
    pub contents_file_size_touch: i64,
    #[serde(with = "either::serde_untagged")]
    pub trials: Either<Vec<File>, bool>,
    #[serde(with = "either::serde_untagged")]
    pub trials_touch: Either<Vec<File>, bool>,
    pub movies: bool,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub epub_sample: Option<EpubSample>,
    pub sample_type: String,
    pub is_viewable_sample: bool,
    pub campaign_id: Option<i64>,
    pub official_price: i64,
    pub official_price_without_tax: i64,
    pub official_trade_price: i64,
    pub official_price_usd: f64,
    pub official_price_eur: f64,
    pub discount_rate: Option<i64>,
    pub is_discount_work: bool,
    pub discount_access_key: Option<String>,
    pub discount_layout: Option<String>,
    pub discount_trade_price_type: Option<String>,
    pub campaign_start_date: Option<String>,
    pub campaign_end_date: Option<String>,
    pub is_show_campaign_end_date: bool,
    pub chobits: bool,
    // pub work_options: HashMap<String, WorkOption>,
    pub work_options: Value,
    pub gift: Vec<String>,
    pub work_rentals: Vec<String>,
    pub is_rental_work: bool,
    pub translation_info: TranslationInfo,
    pub display_order: i64,
    pub is_oauth_work: Option<bool>,
    pub is_show_rate: bool,
    pub rate_average_star: i64,
    pub rate_count_detail: HashMap<String, i64>,
    pub rank_total: Option<i64>,
    pub rank_total_date: Option<String>,
    pub rank_year: Option<i64>,
    pub rank_year_date: Option<i64>,
    pub rank_month: Option<i64>,
    pub rank_month_date: Option<String>,
    pub rank_week: Option<i64>,
    pub rank_week_date: Option<String>,
    pub rank_day: Option<i64>,
    pub rank_day_date: Option<String>,
    pub is_pack_child: bool,
    pub work_pack_parent: Vec<String>,
    pub is_pack_parent: bool,
    pub work_pack_children: Vec<String>,
    pub pack_type: Option<String>,
    pub is_voice_pack: bool,
    pub voice_pack_parent: Vec<String>,
    pub voice_pack_child: Vec<String>,
    pub free: bool,
    pub free_only: bool,
    pub free_end_date: Option<bool>,
    pub has_free_download: bool,
    pub limited_free_terms: Vec<String>,
    #[serde(default)]
    #[serde(rename = "creaters")]
    pub creators: Option<Creators>,
    pub title_id: Option<String>,
    pub title_name: Option<String>,
    pub title_volumn: Option<i64>,
    pub title_work_labeling: Option<String>,
    pub title_work_display_order: Option<i64>,
    pub title_work_count: Option<i64>,
    pub is_title_completed: bool,
    pub title_latest_workno: Option<String>,
    pub title_price_low: Option<String>,
    pub title_price_high: Option<String>,
    pub is_title_pointup: Option<String>,
    pub title_point_rate: Option<String>,
    pub is_title_discount: Option<String>,
    pub is_title_reserve: Option<String>,
    pub reserve_work: Option<ReserveWork>,
    pub is_reserve_work: bool,
    pub is_reservable: bool,
    pub is_downloadable_reserve_work: bool,
    pub bonus_workno: bool,
    pub bonus_work: Option<String>,
    pub is_bonus_work: bool,
    pub is_downloadable_bonus_work: bool,
    pub parent_reserve_workno: bool,
    pub book_type: Option<BookType>,
    pub is_bl: bool,
    pub is_tl: bool,
    pub is_drama_work: bool,
    pub is_display_notice: bool,
    pub touch_style1: Vec<Value>,
    pub is_bulkbuy: bool,
    pub bulkbuy_key: Option<String>,
    pub bulkbuy_title: Option<String>,
    pub bulkbuy_per_items: i64,
    pub bulkbuy_start: Option<String>,
    pub bulkbuy_end: Option<String>,
    pub bulkbuy_price: i64,
    pub bulkbuy_price_tax: i64,
    pub bulkbuy_price_without_tax: i64,
    pub bulkbuy_trade_price: i64,
    pub bulkbuy_trade_price_tax: i64,
    pub bulkbuy_trade_price_without_tax: i64,
    pub bulkbuy_trade_price_type: String,
    pub bulkbuy_discount_rate: i64,
    pub bulkbuy_point_rate: i64,
    pub bulkbuy_point: i64,
    pub genres: Vec<Genre>,
    pub custom_genres: Vec<CustomGenre>,
    pub editions: Vec<Value>,
    pub language_editions: Vec<Value>,
    pub display_options: Vec<String>,
    pub work_browse_setting: Value,
    pub is_limit_work: bool,
    pub is_limit_sales: bool,
    pub is_limit_in_stock: bool,
    pub limit_start_date: Option<String>,
    pub limit_end_date: Option<String>,
    pub limit_dl_count: i64,
    pub limit_start_dl_count: i64,
    pub limit_display_type: Option<String>,
    pub limit_note: Option<String>,
    pub is_timesale_work: bool,
    pub timesale_dl_count: i64,
    pub timesale_limit_dl_count: Option<String>,
    pub timesale_stock: i64,
    pub timesale_start_date: Option<String>,
    pub timesale_end_date: Option<String>,
    pub timesale_price: i64,
    pub timesale_trade_price: i64,
    pub update_date: String,
    pub locale_price: HashMap<String, f64>,
    pub locale_official_price: HashMap<String, f64>,
    pub locale_price_str: HashMap<String, String>,
    pub locale_official_price_str: HashMap<String, String>,
    pub given_coupons_by_buying: Vec<String>,
    pub author: Option<Vec<Author>>,
    pub authors: Option<Vec<Author>>,
    pub product_dir: String,
    pub srcset: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    pub workno: String,
    pub r#type: String,
    pub file_name: String,
    pub file_size: String,
    pub file_size_unit: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub hash: Option<String>,
    pub display_mode: String,
    pub update_date: String,
    pub id: String,
    #[serde(rename = "upper(work_files.type)")]
    pub upper_work_files_type: String,
    pub extension: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub workno: Option<String>,
    pub r#type: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<String>,
    pub file_size_unit: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub hash: Option<String>,
    pub display_mode: Option<String>,
    pub update_date: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "upper(work_files.type)")]
    pub upper_work_files_type: Option<String>,
    pub extension: Option<String>,
    pub relative_url: Option<String>,
    pub path_short: Option<String>,
    pub url: String,
    pub resize_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpubSample {
    pub volume_type: String,
    pub volume: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkOption {
    pub id: String,
    pub options_id: String,
    pub value: String,
    pub name: String,
    pub name_en: String,
    pub display_sentence: Option<String>,
    pub display_sentence_en: Option<String>,
    pub category_id: String,
    pub category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomGenre {
    pub genre_key: String,
    pub lang: String,
    pub name: String,
    pub layout: Value,
    pub status: String,
    pub is_active: i64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationInfo {
    pub is_translation_agree: bool,
    pub is_volunteer: bool,
    pub is_original: bool,
    pub is_parent: bool,
    pub is_child: bool,
    pub original_workno: Option<String>,
    pub parent_workno: Option<String>,
    pub child_worknos: Vec<String>,
    pub lang: Option<String>,
    pub production_trade_price_rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReserveWork {
    pub workno: String,
    pub status: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub bonus_workno: Option<String>,
    pub bonus_end_date: Option<String>,
    pub tag: Option<String>,
    pub display_order: String,
    pub note: Option<String>,
    pub del_flg: String,
    pub update_date: String,
    pub insert_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookType {
    pub id: String,
    pub options_id: String,
    pub value: String,
    pub name: String,
    pub name_en: String,
    pub display_sentence: Option<String>,
    pub display_sentence_en: Option<String>,
    pub category_id: String,
    pub category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Genre {
    pub name: String,
    pub id: i64,
    pub search_val: String,
    pub name_base: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkBrowseSetting {
    pub play_encode_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub author_id: String,
    pub sort_id: String,
    pub author_role_id: String,
    pub author_name_kana: String,
    pub other: Option<String>,
    pub url: String,
    pub author_name: String,
    pub author_role_name: String,
    pub author_role_omission_name: String,
    pub upper_books_work_author_author_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creators {
    created_by: Option<Vec<Creator>>,
    voice_by: Option<Vec<Creator>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    id: String,
    name: String,
    classification: String,
    sub_classification: Option<String>,
}

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
    pub async fn get_product_api(&self, id: &str) -> Result<ProductApiContent> {
        let json = self
            .get(&format!("/api/=/product.json?workno={}", id))
            .await?;
        let jd = &mut serde_json::Deserializer::from_str(&json);
        let result: std::result::Result<ProductApiResult, _> = serde_path_to_error::deserialize(jd);

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

#[cfg(test)]
mod test {
    use anyhow::Context;
    use rand::Rng;

    use super::Genre;
    use crate::{
        interface::{AgeCategory, WorkType},
        DlsiteClient,
    };
    use test_case::test_case;

    #[tokio::test]
    async fn get_product_api_1_content() {
        let client = DlsiteClient::default();
        let res = client.get_product_api("RJ403038").await.unwrap();

        assert_eq!(res.workno, "RJ403038");
        assert_eq!(
            res.work_name,
            "【ブルーアーカイブ】ユウカASMR～頑張るあなたのすぐそばに～".to_string()
        );
        assert_eq!(res.maker_name, "Yostar");
        assert_eq!(res.circle_id.clone().unwrap(), "RG62982");
        assert_eq!(res.work_type, WorkType::SOU);
    }

    #[tokio::test]
    async fn get_product_api_2() {
        let client = DlsiteClient::default();
        let res = client
            .get_product_api("RJ01017217")
            .await
            .context("Failed to get product info");
        let res = res.unwrap();
        assert_eq!(res.workno, "RJ01017217".to_string());
        assert_eq!(
            res.work_name,
            "【イヤーキャンドル】道草屋-なつな3-たぬさんこんにちは【ずぶ濡れシャンプー】"
                .to_string()
        );
        assert_eq!(res.maker_name, "桃色CODE");
        assert_eq!(res.circle_id, Some("RG24350".to_string()));
        //
        assert_eq!(res.work_type, WorkType::SOU);
        assert_eq!(res.age_category, AgeCategory::Adult);
        let creators = res.creators.unwrap();
        let voice_by = creators.voice_by.unwrap();
        let created_by = creators.created_by.unwrap();
        assert_eq!(voice_by[0].name, "丹羽うさぎ");
        assert_eq!(voice_by[1].name, "藤堂れんげ");
        assert_eq!(created_by[0].name, "桃鳥");
        assert!(res.genres.contains(&Genre {
            name: "ASMR".to_string(),
            id: 497,
            search_val: "497".to_string(),
            name_base: "ASMR".to_string()
        }));
    }

    #[test_case("RJ01084246"; "otome")]
    #[test_case("VJ01000513"; "soft")]
    #[test_case("RJ411991"; "normal")]
    #[tokio::test]
    async fn get_product_api_success(id: &str) {
        let client = DlsiteClient::default();
        client.get_product_api(id).await.unwrap();
    }

    #[tokio::test]
    async fn get_product_api_env() {
        if let Some(id) = std::option_env!("PRODUCT_TEST_ID") {
            let client = DlsiteClient::default();
            client.get_product_api(id).await.unwrap();
        }
    }

    #[tokio::test]
    async fn get_product_api_rand_rj() {
        let mut rng = rand::thread_rng();
        let mut i = 0;
        loop {
            let id = rng.gen_range(100000..1000000);
            let id = if id >= 1000000 {
                format!("RJ0{}", id)
            } else {
                format!("RJ{}", id)
            };
            let client = DlsiteClient::default();
            println!("Testing for {}", id);
            let pre_req = client
                .get(&format!("/api/=/product.json?workno={}", id))
                .await
                .unwrap();
            if pre_req.trim() == "[]" {
                println!("Testing for {}: Invalid id", id);
                continue;
            }
            client.get_product_api(&id).await.unwrap();

            i += 1;
            if i >= 5 {
                break;
            }
        }
    }
}
