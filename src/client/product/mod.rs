//! Interfaces related to product. For more information, see [`ProductClient`].

use std::collections::HashMap;

use super::genre::Genre;
use crate::{
    client::common::{AgeCategory, WorkType},
    error::Result,
    utils::ToParseError as _,
    DlsiteClient, DlsiteError,
};
use ajax::ProductAjax;
use chrono::NaiveDate;

pub mod ajax;
pub mod html;
pub mod review;
#[cfg(test)]
mod test;

/// Client to retrieve DLsite product data using 'scraping' method.
///
/// # Scraping vs API
///
/// There are two way to get product data from DLsite.
/// 1. the 'scraping' method (this client's method): This reproduces the behavior of browsing a DLSite site in a browser. That is, it retrieves the DLsite html file and scrapes it.
/// 2. the 'api' method (in [`super::product_api::ProductApiClient`]): this uses directly the API that DLsite provides for their app.
///    This is a json api, so it is easy to parse, but has the problem that some data can only be obtained by the scrapping method,
///    and if DLsite changes their api, it will be unusable until this library follows suit.
///
/// ## Details about scraping method
/// Actually, the 'scraping' method does more than just scraping html:
/// when browsing the DLsite, some information, such as product details and review information,
/// is retrieved from a separate ajax api.
///
/// Scraping is specifically performed by the following process
/// 1. parsing HTML to obtain basic product information
/// 2. retrieve detailed product information and related products via an API
///    (this is completely different from the api used in the api method, and is referred to as the 'ajax api' in this crate.)
/// 3. retrieve review information by another api (called 'review api')
///
/// So this client has a `get_all` method to do all 1-3 and get the complete information, and each method to do only each.
#[derive(Clone, Debug)]
pub struct ProductClient<'a> {
    pub(crate) c: &'a DlsiteClient,
}

/// A product on DLsite.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub work_type: WorkType,
    pub released_at: NaiveDate,
    pub age_rating: Option<AgeCategory>,
    pub genre: Vec<Genre>,
    pub circle_id: String,
    pub circle_name: String,
    pub price: i32,
    pub series: Option<String>,
    pub sale_count: Option<i32>,
    pub review_count: Option<i32>,
    pub rating: Option<f32>,
    pub rate_count: Option<i32>,
    pub images: Vec<String>,
    pub people: ProductPeople,
    pub reviewer_genre: Vec<(Genre, i32)>,
    pub file_format: Vec<String>,
    pub file_size: Option<String>,
    pub product_format: Vec<String>,
}

/// People who contributed to a product on DLsite.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ProductPeople {
    pub author: Option<Vec<String>>,
    pub scenario: Option<Vec<String>>,
    pub illustrator: Option<Vec<String>>,
    pub voice_actor: Option<Vec<String>>,
}

impl<'a> ProductClient<'a> {
    /// Get full information about a product. For more detail, see documentation of [`ProductClient`].
    ///
    /// # Arguments
    /// * `product_id` - The product ID to get information about. Example: `RJ123456`. NOTE: This must be capitalized.
    ///
    /// # Example
    /// ```
    /// use dlsite::DlsiteClient;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = DlsiteClient::default();
    ///     let product = client.product().get_all("RJ123456").await.unwrap();
    ///     println!("{:#?}", product);
    /// }
    /// ```
    pub async fn get_all(&self, product_id: &str) -> Result<Product> {
        let (html_data, ajax_data, review_data) = tokio::try_join!(
            self.get_html(product_id),
            self.get_ajax(product_id),
            self.get_review(product_id, 6, 1, true, review::ReviewSortOrder::New)
        )?;

        Ok(Product {
            id: product_id.to_string(),
            title: ajax_data.work_name,
            work_type: ajax_data.work_type,
            released_at: html_data.released_at,
            age_rating: html_data.age_rating,
            genre: html_data.genre,
            series: html_data.series,
            circle_name: html_data.circle_name,
            circle_id: html_data.circle_id,
            price: ajax_data.price,
            rating: ajax_data.rate_average_2dp,
            rate_count: ajax_data.rate_count,
            sale_count: ajax_data.dl_count,
            review_count: ajax_data.review_count,
            images: html_data.images,
            people: html_data.people,
            reviewer_genre: review_data.reviewer_genre_list.unwrap_or_default(),
            file_format: html_data.file_format,
            file_size: html_data.file_size,
            product_format: html_data.product_format,
        })
    }

    /// Scrapes the HTML page of a product and parses it.
    #[tracing::instrument(err)]
    pub async fn get_html(&self, product_id: &str) -> Result<html::ProductHtml> {
        let path = format!("/work/=/product_id/{}", product_id);
        let html = self.c.get(&path).await?;
        let html = scraper::Html::parse_document(&html);

        html::parse_product_html(&html)
    }

    /// Fetch detailed product information using 'ajax api'.
    pub async fn get_ajax(&self, product_id: &str) -> Result<ProductAjax> {
        let path = format!("/product/info/ajax?product_id={}", product_id);
        let ajax_json_str = self.c.get(&path).await?;

        let mut json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;
        let product = json
            .remove(product_id)
            .ok_or_else(|| DlsiteError::Parse("Failed to parse ajax json".to_string()))?;

        Ok(product)
    }

    /// Fetch detailed multiple products information using 'ajax api'.
    ///
    /// It is more efficient to use this method than calling `get_ajax` multiple times.
    #[tracing::instrument(err)]
    pub async fn get_ajax_multiple(
        &self,
        product_ids: Vec<&str>,
    ) -> Result<HashMap<String, ProductAjax>> {
        let path = format!("/product/info/ajax?product_id={}", product_ids.join(","));
        let ajax_json_str = self.c.get(&path).await?;

        let json: HashMap<String, ProductAjax> = serde_json::from_str(&ajax_json_str)?;

        Ok(json)
    }

    /// Get product reviews and related informations using 'review api'.
    ///
    /// # Arguments
    /// * `product_id` - Product ID.
    /// * `limit` - Number of reviews to get.
    /// * `page` - Page number.
    /// * `mix_pickup` - Mixes picked up review. To get user genre, this must be true.
    /// * `order` - Sort order of reviews.
    ///
    /// # Returns
    /// Product reviews and related informations.
    #[tracing::instrument(err, skip_all)]
    pub async fn get_review(
        &self,
        product_id: &str,
        limit: u32,
        page: u32,
        mix_pickup: bool,
        order: review::ReviewSortOrder,
    ) -> Result<review::ProductReview> {
        let order_str = match order {
            review::ReviewSortOrder::New => "regist_d",
            review::ReviewSortOrder::Top => "top",
        };

        let path = format!(
            "/api/review?product_id={}&limit={}&mix_pickup={}&page={}&order={}&locale=ja_JP",
            product_id, limit, mix_pickup, page, order_str
        );
        let json_str = self.c.get(&path).await?;
        let json: serde_json::Value = serde_json::from_str(&json_str)?;

        if !json["is_success"]
            .as_bool()
            .to_parse_error("Failed to parse revire json")?
        {
            let message = json["error_msg"]
                .as_str()
                .unwrap_or("Failed to get error message");
            return Err(crate::DlsiteError::Server(format!(
                "Failed to get review: {}",
                message
            )));
        }

        let json: review::ProductReview = serde_json::from_value(json)?;
        Ok(json)
    }
}
