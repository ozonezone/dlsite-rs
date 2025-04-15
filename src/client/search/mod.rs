//! Interfaces related to search feature. For more information, see [`SearchClient`].

pub(crate) mod macros;
pub mod options;

use scraper::{Html, Selector};
use serde::Deserialize;

use crate::{
    client::common::{AgeCategory, WorkType},
    error::Result,
    utils::ToParseError,
    DlsiteClient,
};

use self::options::SearchProductQuery;

/// Client to search products on DLsite.
pub struct SearchClient<'a> {
    pub(crate) c: &'a DlsiteClient,
}

#[derive(Deserialize)]
struct SearchPageInfo {
    count: i32,
}

#[derive(Deserialize)]
struct SearchAjaxResult {
    search_result: String,
    page_info: SearchPageInfo,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SearchProductItem {
    pub id: String,
    pub title: String,
    pub creator: Option<String>,
    pub creator_omitted: Option<bool>,
    pub circle_name: String,
    pub circle_id: String,
    pub dl_count: Option<i32>,
    pub rate_count: Option<i32>,
    pub review_count: Option<i32>,
    pub price_original: i32,
    pub price_sale: Option<i32>,
    pub age_category: AgeCategory,
    pub work_type: crate::client::common::WorkType,
    pub thumbnail_url: String,
    pub rating: Option<f32>, // pub image_url: Option<String>,
}

#[derive(Debug)]
pub struct SearchResult {
    pub products: Vec<SearchProductItem>,
    pub count: i32,
    pub query_path: String,
}
fn parse_count_str(str: &str) -> Result<i32> {
    str.replace(['(', ')', ','], "")
        .parse()
        .to_parse_error("Failed to parse string to count")
}

fn parse_num_str(str: &str) -> Result<i32> {
    str.replace(',', "")
        .parse()
        .to_parse_error("Failed to parse string to number")
}

impl<'a> SearchClient<'a> {
    /// Search products on DLsite.
    ///
    /// # Arguments
    /// * `options` - Struct of search options.
    ///
    /// # Example
    /// ```
    /// use dlsite::{DlsiteClient, client::search::options::*};
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = DlsiteClient::default();
    ///     let product = client
    ///         .search()
    ///         .search_product(&SearchProductQuery {
    ///             sex_category: Some(vec![SexCategory::Male]),
    ///             keyword: Some("ASMR".to_string()),
    ///             ..Default::default()
    ///         })
    ///         .await
    ///         .expect("Failed to search");
    ///     dbg!(&product);
    /// }
    /// ```
    pub async fn search_product(&self, options: &SearchProductQuery) -> Result<SearchResult> {
        let query_path = options.to_path();
        let json = self.c.get(&query_path).await?;
        let json = serde_json::from_str::<SearchAjaxResult>(&json)?;
        let html = json.search_result;
        let count = json.page_info.count;

        let products = parse_search_html(&html)?;

        Ok(SearchResult {
            products,
            count,
            query_path,
        })
    }
}

pub(crate) fn parse_search_html(html: &str) -> Result<Vec<SearchProductItem>> {
    let html = Html::parse_fragment(html);
    let mut result: Vec<SearchProductItem> = vec![];

    for item_element in html.select(&Selector::parse("#search_result_img_box > li").unwrap()) {
        let product_id_e = item_element
            .select(&Selector::parse("div[data-product_id]").unwrap())
            .next()
            .to_parse_error("Failed to find data element")?
            .value();
        let maker_e = item_element
            .select(&Selector::parse(".maker_name a").unwrap())
            .next()
            .to_parse_error("Failed to find maker element")?;
        let author_e = item_element
            .select(&Selector::parse(".author").unwrap())
            .next();

        let price_e = item_element
            .select(&Selector::parse(".work_price .work_price_base").unwrap())
            .next()
            .to_parse_error("Failed to find price element")?;
        let original_price_e = item_element
            .select(&Selector::parse(".work_price_wrap .strike .work_price_base").unwrap())
            .next();
        let (sale_price_e, original_price_e) = if let Some(e) = original_price_e {
            (Some(price_e), e)
        } else {
            (None, price_e)
        };
        let id = product_id_e
            .attr("data-product_id")
            .to_parse_error("Failed to get product id")?
            .to_string();

        result.push(SearchProductItem {
            id: id.clone(),
            title: item_element
                .select(&Selector::parse(".work_name a[title]").unwrap())
                .next()
                .to_parse_error("Failed to get title")?
                .value()
                .attr("title")
                .unwrap()
                .to_string(),
            age_category: {
                if let Some(e) = item_element
                    .select(&Selector::parse(".work_genre span").unwrap())
                    .next()
                {
                    let title = e.value().attr("title");
                    if let Some(title) = title {
                        match title {
                            "全年齢" => AgeCategory::General,
                            "R-15" => AgeCategory::R15,
                            _ => {
                                return Err(crate::DlsiteError::Parse(
                                    "Age category parse error: invalid title".to_string(),
                                ))
                            }
                        }
                    } else {
                        return Err(crate::DlsiteError::Parse(
                            "Age category parse error".to_string(),
                        ));
                    }
                } else {
                    AgeCategory::Adult
                }
            },
            circle_name: maker_e.text().next().unwrap_or("").to_string(),
            circle_id: maker_e
                .value()
                .attr("href")
                .to_parse_error("Failed to get maker link")?
                .split('/')
                .next_back()
                .to_parse_error("Invalid url")?
                .split('.')
                .next()
                .to_parse_error("Failed to find maker id")?
                .to_string(),
            creator: {
                if let Some(creator_e) = author_e {
                    let name = creator_e
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .to_parse_error("Failed to find creator")?
                        .text()
                        .next()
                        .to_parse_error("Failed to find creator")?
                        .to_string();
                    Some(name)
                } else {
                    None
                }
            },
            creator_omitted: {
                if let Some(creator_e) = author_e {
                    let omitted = creator_e
                        .value()
                        .attr("class")
                        .to_parse_error("Failed to find creator")?
                        .split(" ")
                        .any(|x| x == "omit");
                    Some(omitted)
                } else {
                    None
                }
            },
            dl_count: {
                if let Some(e) = item_element
                    .select(&Selector::parse(".work_dl span[class*=\"dl_count\"]").unwrap())
                    .next()
                {
                    Some(
                        e.text()
                            .next()
                            .to_parse_error("Failed to get dl count")?
                            .replace(',', "")
                            .parse()
                            .to_parse_error("Invalid dl count")?,
                    )
                } else {
                    None
                }
            },
            rate_count: {
                if let Some(e) = item_element
                    .select(&Selector::parse(".work_dl span[class*=\"dl_count\"]").unwrap())
                    .next()
                {
                    Some(parse_count_str(
                        e.text().next().to_parse_error("Failed to get rate count")?,
                    )?)
                } else {
                    None
                }
            },
            review_count: {
                if let Some(e) = item_element
                    .select(&Selector::parse(".work_review div a").unwrap())
                    .next()
                {
                    Some(parse_count_str(
                        e.text()
                            .next()
                            .to_parse_error("Failed to get review count")?,
                    )?)
                } else {
                    None
                }
            },
            price_original: parse_num_str(
                original_price_e
                    .text()
                    .next()
                    .to_parse_error("Failed to find price")?,
            )?,
            price_sale: {
                match sale_price_e {
                    Some(e) => Some(parse_num_str(
                        e.text().next().to_parse_error("Failed to find price")?,
                    )?),
                    None => None,
                }
            },
            work_type: item_element
                .select(&Selector::parse(".work_category").unwrap())
                .next()
                .to_parse_error("Failed to find work category")?
                .value()
                .attr("class")
                .to_parse_error("Failed to find worktype")?
                .split(' ')
                .find_map(|c| {
                    if let Some(c) = c.strip_prefix("type_") {
                        if let Ok(wt) = c.parse::<WorkType>() {
                            if let WorkType::Unknown(_) = wt {
                                return None;
                            } else {
                                return Some(wt);
                            }
                        }
                    }
                    None
                })
                .unwrap_or(crate::client::common::WorkType::Unknown("".to_string())),
            thumbnail_url: {
                let img_e = item_element
                    .select(&Selector::parse(".work_thumb_inner > img").unwrap())
                    .next()
                    .to_parse_error("Failed to find thumbnail")?;

                let src = img_e.value().attr("src");
                let data_src = img_e.value().attr("data-src");
                match (src, data_src) {
                    (Some(src), _) => format!("https:{}", src),
                    (_, Some(data_src)) => format!("https:{}", data_src),
                    (_, _) => {
                        return Err(crate::DlsiteError::Parse(
                            "Failed to find thumbnail".to_string(),
                        ))
                    }
                }
            },
            rating: {
                if let Some(e) = item_element
                    .select(&Selector::parse(".work_rating .star_rating").unwrap())
                    .next()
                {
                    e.value()
                        .attr("class")
                        .expect("Failed to get rating")
                        .split(' ')
                        .find_map(|c| {
                            if let Some(c) = c.strip_prefix("star_") {
                                if let Ok(r) = c.parse::<f32>() {
                                    return Some(r / 10.0);
                                }
                            }
                            None
                        })
                } else {
                    None
                }
            }, // image_url: {
               //     if let Some(e) = item_element
               //         .select(&Selector::parse(".work_img_popover img").unwrap())
               //         .next()
               //     {
               //         Some(
               //             e.value()
               //                 .attr("src")
               //                 .to_parse_error("Failed to get image url")?
               //                 .to_string(),
               //         )
               //     } else {
               //         None
               //     }
               // },
        })
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::client::{common::WorkType, search::options::*, DlsiteClient};

    #[tokio::test]
    async fn search_product_1() {
        let client = DlsiteClient::default();
        let res = client
            .search()
            .search_product(&super::SearchProductQuery {
                sex_category: Some(vec![SexCategory::Male]),
                keyword: Some("ユウカASMR".to_string()),
                ..Default::default()
            })
            .await
            .expect("Failed to search");

        assert!(res.products.len() >= 8);
        assert!(res.count >= 8);

        res.products
            .iter()
            .find(|r| r.id == "RJ403038")
            .expect("Expected to find RJ403038");

        res.products.iter().for_each(|r| {
            if r.id == "RJ403038" {
                assert_eq!(1320, r.price_original);
                assert!(r.dl_count.unwrap() > 62000);
                assert!(r.rate_count.is_some());
                assert!(r.review_count.is_some());
                assert!(r.rating.is_some());
                assert!(r.rating.is_some());
                assert_eq!("RG62982", r.circle_id);
                assert_eq!("Yostar", r.circle_name);
                assert_eq!(WorkType::SOU, r.work_type);
                assert_eq!("春花らん", r.creator.as_ref().unwrap());
                assert!(!r.creator_omitted.unwrap());
            }
        });
    }

    #[tokio::test]
    async fn search_product_2() {
        let client = DlsiteClient::default();
        let mut opts = super::SearchProductQuery {
            sex_category: Some(vec![SexCategory::Male]),
            order: Some(Order::Trend),
            per_page: Some(50),
            ..Default::default()
        };

        let res = client
            .search()
            .search_product(&opts)
            .await
            .expect("Failed to search page 1");
        res.products.iter().for_each(|i| {
            url::Url::parse(&i.thumbnail_url).expect("Failed to parse url");
            dbg!(&i);
        });
        assert_eq!(50, res.products.len());

        opts.page = Some(2);
        let res = client
            .search()
            .search_product(&opts)
            .await
            .expect("Failed to search page 2");
        res.products.iter().for_each(|i| {
            url::Url::parse(&i.thumbnail_url).expect("Failed to parse url");
        });
        assert_eq!(50, res.products.len());
    }
}
