pub(crate) mod macros;
pub mod options;

use scraper::{Html, Selector};
use serde::Deserialize;

use crate::{
    interface::{AgeCategory, WorkType},
    utils::ToParseError,
    DlsiteClient, Result,
};

use self::options::ProductSearchOptions;

#[derive(Deserialize)]
struct SearchAjaxResult {
    search_result: String,
    // page_info: serde_json::Value,
}

#[derive(Debug)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub circle_name: String,
    pub circle_id: String,
    pub dl_count: Option<i32>,
    pub rate_count: Option<i32>,
    pub review_count: Option<i32>,
    pub price_original: i32,
    pub price_sale: Option<i32>,
    pub age_category: AgeCategory,
    pub work_type: crate::interface::WorkType,
    pub thumbnail_url: String,
    // pub image_url: Option<String>,
}

fn parse_count_str(str: &str) -> Result<i32> {
    str.replace(['(', ')', ','], "")
        .parse()
        .to_parse_error("Failed to parse string")
}

fn parse_num_str(str: &str) -> Result<i32> {
    str.replace(',', "")
        .parse()
        .to_parse_error("Failed to parse string")
}

impl DlsiteClient {
    /// Search products on DLsite.
    ///
    /// # Arguments
    /// * `options` - Struct of search options.
    pub async fn search_product(
        &self,
        options: &ProductSearchOptions,
    ) -> Result<Vec<SearchResult>> {
        let json = self.get(&options.to_path()).await?;
        let json = serde_json::from_str::<SearchAjaxResult>(&json)?;
        let html = json.search_result;

        parse_search_html(&html)
    }
}

pub(crate) fn parse_search_html(html: &str) -> Result<Vec<SearchResult>> {
    let html = Html::parse_fragment(html);
    let mut result: Vec<SearchResult> = vec![];

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

        let price_e = item_element
            .select(&Selector::parse(".work_price").unwrap())
            .next()
            .to_parse_error("Failed to find price element")?;
        let original_price_e = item_element
            .select(&Selector::parse(".work_price_wrap .strike").unwrap())
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

        result.push(SearchResult {
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
                                return Err(crate::DlsiteError::ParseError(
                                    "Age category parse error: invalid title".to_string(),
                                ))
                            }
                        }
                    } else {
                        return Err(crate::DlsiteError::ParseError(
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
                .last()
                .to_parse_error("Invalid url")?
                .split('.')
                .next()
                .to_parse_error("Failed to find maker id")?
                .to_string(),
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
                .unwrap_or(crate::interface::WorkType::Unknown("".to_string())),
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
                        return Err(crate::DlsiteError::ParseError(
                            "Failed to find thumbnail".to_string(),
                        ))
                    }
                }
            },
            // image_url: {
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
    use crate::{search::options::*, DlsiteClient};

    #[tokio::test]
    async fn search_product_1() {
        let client = DlsiteClient::default();
        let res = client
            .search_product(&super::ProductSearchOptions {
                sex_category: Some(vec![SexCategory::Male]),
                keyword: Some("ユウカASMR".to_string()),
                ..Default::default()
            })
            .await
            .expect("Failed to search");

        assert!(res.len() >= 8);

        res.iter()
            .find(|r| r.id == "RJ403038")
            .expect("Expected to find RJ403038");

        res.iter().for_each(|r| {
            if r.id == "RJ403038" {
                assert_eq!(1320, r.price_original);
                assert!(r.dl_count.unwrap() > 62000);
                assert!(r.rate_count.is_some());
                assert!(r.review_count.is_some());
                assert_eq!("RG62982", r.circle_id);
                assert_eq!("Yostar", r.circle_name);
                assert_eq!(crate::interface::WorkType::SOU, r.work_type);
            }
        });
    }

    #[tokio::test]
    async fn search_product_2() {
        let client = DlsiteClient::default();
        let mut opts = super::ProductSearchOptions {
            sex_category: Some(vec![SexCategory::Male]),
            order: Some(Order::Trend),
            per_page: Some(50),
            ..Default::default()
        };

        let res = client
            .search_product(&opts)
            .await
            .expect("Failed to search page 1");
        res.iter().for_each(|i| {
            url::Url::parse(&i.thumbnail_url).expect("Failed to parse url");
            dbg!(&i);
        });
        assert_eq!(50, res.len());

        opts.page = Some(2);
        let res = client
            .search_product(&opts)
            .await
            .expect("Failed to search page 2");
        res.iter().for_each(|i| {
            url::Url::parse(&i.thumbnail_url).expect("Failed to parse url");
        });
        assert_eq!(50, res.len());
    }
}
