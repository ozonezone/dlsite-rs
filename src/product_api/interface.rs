use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{formats::PreferOne, serde_as, DefaultOnError, OneOrMany};

use crate::interface::{AgeCategory, FileType, WorkCategory, WorkType};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Either<T, V> {
    Left(T),
    Right(V),
}

pub type StrOrBool = Either<String, bool>;
pub type ArrOrSingle<T> = Either<Vec<T>, T>;
pub type HashMapOrArr<T> = Either<HashMap<String, T>, Vec<T>>;

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct LimitedFree {
    pub end_date: String,
    pub id: i32,
    pub original_workno: Option<String>,
    pub workno: String,
    pub start_date: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct WorkPackChild {
    pub inservice: i32,
    pub product_id: String,
    pub product_name: String,
    pub work_name: String,
    pub workno: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct LanguageEdition {
    pub display_order: i32,
    pub edition_id: i32,
    pub edition_type: String,
    pub label: String,
    pub lang: String,
    pub workno: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Options {
    pub end_date: Option<String>,
    pub start_date: Option<String>,
    pub frame_sort: FrameSort,
    pub pickups: Option<Vec<String>>,
    pub timesale_search: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct FrameSort {
    pub discount: String,
    pub pickup: String,
    pub pickup_free: String,
    pub related: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Discount {
    pub access_key: Option<String>,
    pub campaign_id: i32,
    pub campaign_price: i32,
    pub del_flg: String,
    pub discount_rate: i32,
    pub end_date: i32,
    pub id: String,
    pub insert_date: String,
    pub insert_id: Option<String>,
    pub limit_dl_count: Option<i32>,
    pub note: Option<Value>,
    pub options: ArrOrSingle<Options>,
    pub restore_price: i32,
    pub restore_trade_price: i32,
    pub show_end_date_days: String,
    pub start_date: i32,
    pub status: String,
    pub title: String,
    pub trade_price_type: String,
    pub update_date: String,
    pub update_id: Option<String>,
    pub workno: String,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Coupling {
    pub coupling: String,
    pub coupling_id: String,
    pub show_coupling: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
    pub coupling: Vec<Coupling>,
    pub cpu: Option<String>,
    pub default_point: i64,
    pub directed_by: Option<String>,
    pub directx: Option<String>,
    pub discount: Option<Discount>,
    pub dist_flag: i64,
    pub dl_format: i64,
    pub etc: Option<String>,
    pub file_date: Option<String>,
    pub file_size: Option<String>,
    pub file_type: FileType,
    pub file_type_string: Option<String>,
    pub file_type_special: Option<String>,
    pub gallery_mode: Option<String>,
    pub hdd: Option<String>,
    pub h_scene_mode: Option<String>,
    pub intro: Option<String>,
    pub intro_s: Option<String>,
    pub label_id: Option<String>,
    pub label_name: Option<String>,
    pub machine: Option<String>,
    pub machine_string_list: HashMapOrArr<Option<String>>,
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
    pub work_name_kana: Option<String>,
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
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    pub image_thum_touch: Vec<File>,
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
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub trials: Option<Vec<File>>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub trials_touch: Option<Vec<File>>,
    pub movies: Either<bool, Vec<File>>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub epub_sample: Option<EpubSample>,
    pub sample_type: String,
    pub is_viewable_sample: bool,
    pub campaign_id: Option<i64>,
    pub official_price: i64,
    pub official_price_without_tax: i64,
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
    /// ex. {"C84": {name: "コミックマーケット84", ...}, ...}
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub work_options: Option<HashMap<String, WorkOption>>,
    pub gift: Vec<String>,
    pub work_rentals: Vec<String>,
    pub is_rental_work: bool,
    pub translation_info: TranslationInfo,
    pub display_order: Option<i64>,
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
    pub is_pack_parent: bool,
    pub work_pack_children: Either<Vec<String>, HashMap<String, WorkPackChild>>,
    pub pack_type: Option<String>,
    pub is_voice_pack: bool,
    pub voice_pack_parent: Vec<String>,
    pub voice_pack_child: Vec<String>,
    pub free: bool,
    pub free_only: bool,
    pub free_end_date: Option<StrOrBool>,
    pub has_free_download: bool,
    #[serde_as(deserialize_as = "DefaultOnError")]
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
    pub bonus_workno: Either<bool, String>,
    pub bonus_work: Option<String>,
    pub is_bonus_work: bool,
    pub is_downloadable_bonus_work: bool,
    pub parent_reserve_workno: bool,
    pub book_type: Option<BookType>,
    pub is_bl: bool,
    pub is_tl: bool,
    pub is_drama_work: bool,
    pub is_display_notice: bool,
    pub touch_style1: Vec<String>,
    pub is_bulkbuy: bool,
    pub bulkbuy_key: Option<String>,
    pub bulkbuy_title: Option<String>,
    pub bulkbuy_per_items: i64,
    pub bulkbuy_start: Option<String>,
    pub bulkbuy_end: Option<String>,
    pub bulkbuy_price: i64,
    pub bulkbuy_price_tax: i64,
    pub bulkbuy_price_without_tax: i64,
    pub bulkbuy_discount_rate: i64,
    pub bulkbuy_point_rate: i64,
    pub bulkbuy_point: i64,
    pub genres: Vec<Genre>,
    pub custom_genres: Vec<CustomGenre>,
    pub editions: HashMapOrArr<LanguageEdition>,
    pub language_editions: HashMapOrArr<LanguageEdition>,
    pub display_options: Vec<String>,
    pub is_limit_work: bool,
    pub is_limit_sales: bool,
    pub work_browse_setting: HashMapOrArr<WorkBrowseSetting>,
    pub is_limit_in_stock: bool,
    pub limit_start_date: Option<String>,
    pub limit_end_date: Option<String>,
    pub limit_dl_count: i64,
    // pub limit_start_dl_count: i64,
    pub limit_display_type: Option<String>,
    pub limit_note: Option<String>,
    pub is_timesale_work: bool,
    pub timesale_dl_count: i64,
    pub timesale_limit_dl_count: Option<i32>,
    pub timesale_stock: i64,
    pub timesale_start_date: Option<String>,
    pub timesale_end_date: Option<String>,
    pub timesale_price: i64,
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
    pub alt_name_masked: String,
    pub work_pack_parent: Either<Vec<String>, HashMap<String, WorkPackChild>>,
    pub limited_free_terms: ArrOrSingle<LimitedFree>,
    pub limited_free_work: ArrOrSingle<LimitedFree>,
    pub intro_masked: Option<Value>,
    pub limit_sale_id: Option<Value>,
    pub specified_volume_sets: Vec<SpecifiedVolumeSet>,
    pub series_name_masked: Option<String>,
    pub is_ios_only_work: bool,
    pub specified_volume_set_max_discount_rate: Option<Value>,
    pub has_specified_volume_set: bool,
    pub work_name_masked: String,
    pub introductions_masked: Option<Value>,
    pub intro_s_masked: Option<String>,
    pub work_type_special_masked: Option<String>,
    pub title_name_masked: Option<String>,
    pub currency_price: HashMap<String, f64>,
    pub currency_official_price: HashMap<String, f64>,
    pub is_android_or_ios_only_work: bool,
    pub genres_replaced: Vec<Genre>,
    pub limit_sold_dl_count: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct SpecifiedVolumeSet {
    pub discount_price: i32,
    pub start_date: String,
    pub id: i32,
    pub end_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Content {
    pub workno: String,
    pub r#type: String,
    pub file_name: String,
    pub file_size: String,
    pub file_size_unit: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub hash: Option<String>,
    pub display_mode: Option<String>,
    pub update_date: String,
    pub id: String,
    #[serde(rename = "upper(work_files.type)")]
    pub upper_work_files_type: String,
    pub extension: String,
    pub name_url: Option<String>,
    pub title: Option<String>,
    pub filesize: Option<String>,
    pub name: Option<String>,
    pub filepath: Option<String>,
    pub size: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Image {
    pub id: Option<String>,
    pub file_name: Option<String>,
    pub height: Option<String>,
    pub update_date: Option<String>,
    pub file_size: Option<String>,
    pub hash: Option<String>,
    pub display_mode: Option<String>,
    pub relative_url: Option<String>,
    pub file_size_unit: Option<String>,
    pub width: Option<String>,
    pub path_short: Option<String>,
    #[serde(rename = "upper(work_files.type)")]
    pub upper_work_files_type: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub resize_url: Option<String>,
    pub workno: Option<String>,
    pub extension: Option<String>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct EpubSample {
    pub volume_type: String,
    pub volume: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
    pub translation_bonus_langs: HashMapOrArr<TranslationBonus>,
    pub is_translation_bonus_child: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct TranslationBonus {
    pub child_count: i32,
    pub price: i32,
    pub price_in_tax: i32,
    pub price_tax: i32,
    pub recipient_available_count: i32,
    pub recipient_max: i32,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
    pub insert_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Genre {
    pub name: String,
    pub id: i64,
    pub search_val: String,
    pub name_base: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct WorkBrowseSetting {
    pub play_encode_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
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
    #[serde(rename = "upper(books_work_author.author_id)")]
    pub upper_books_work_author_author_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Creators {
    pub created_by: Option<Vec<Creator>>,
    pub voice_by: Option<Vec<Creator>>,
    pub illust_by: Option<Vec<Creator>>,
    pub scenario_by: Option<Vec<Creator>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "unknown-field-error", serde(deny_unknown_fields))]
pub struct Creator {
    pub id: String,
    pub name: String,
    pub classification: String,
    pub sub_classification: Option<String>,
}
