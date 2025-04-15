//! Interfaces related to product. For more information, see [`ProductClient`].

use super::genre::Genre;
use crate::{
    client::common::{AgeCategory, WorkType},
    error::Result,
    utils::ToParseError as _,
    DlsiteClient,
};
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
/// 1. "scraping" method: Using HTML scraping and "ajax" API. This is similar to how a browser gets data.
/// 2. "api" method: Utilizes api used by DLsite app.
///
/// Each method has its own pros and cons. By using first method, you can get more detailed data.
/// But it is slower because it makes multiple requests and also fragile because it depends on HTML structure.
/// By using second method, you can get data faster and more stable. But you can't get some
/// additional data.
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
    /// Get information about a product (also called "work").
    /// This function will make 3 requests to DLsite.
    /// 1. Get the HTML page of the product. This data can be get using [`DlsiteClient::get_product_html`].
    /// 2. Get the AJAX data of the product. This data can be get using [`DlsiteClient::get_product_ajax`].
    /// 3. Get the review data of the product. This data can be get using [`DlsiteClient::get_product_review`].
    ///
    /// Especially, review data can be used as independent information.
    ///
    /// # Arguments
    /// * `product_id` - The product ID to get information about. Example: `RJ123456`. NOTE: This must be capitalized.
    ///
    /// # Example
    /// ```
    /// use dlsite::{DlsiteClient, product::Product};
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = DlsiteClient::default();
    ///     let product = client.get_product("RJ123456").await.unwrap();
    ///     println!("{:#?}", product);
    /// }
    /// ```
    pub async fn get_all(&self, product_id: &str) -> Result<Product> {
        let (html_data, ajax_data, review_data) = tokio::try_join!(
            self.get_html(product_id),
            self.get_product_ajax(product_id),
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

    /// Scrapes the HTML page of a product and parses it into a [`html::ProductHtml`] struct.
    #[tracing::instrument(err)]
    pub async fn get_html(&self, product_id: &str) -> Result<html::ProductHtml> {
        let path = format!("/work/=/product_id/{}", product_id);
        let html = self.c.get(&path).await?;
        let html = scraper::Html::parse_document(&html);

        html::parse_product_html(&html)
    }

    /// Get product reviews and related informations using ajax api.
    ///
    /// # Arguments
    /// * `product_id` - Product ID.
    /// * `mix_pickup` - Mixes picked up review. To get user genre, this must be true.
    /// * `order` - Sort order of reviews.
    /// * `limit` - Number of reviews to get.
    /// * `page` - Page number.
    ///
    /// # Returns
    /// * `ProductReview` - Product reviews and related informations.
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
            return Err(crate::DlsiteError::ServerError(format!(
                "Failed to get review: {}",
                message
            )));
        }

        let json: review::ProductReview = serde_json::from_value(json)?;
        Ok(json)
    }
}
