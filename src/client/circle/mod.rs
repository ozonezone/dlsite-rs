//! Interfaces related to circle. For more information, see [`CircleClient`].

pub mod options;

use scraper::{Html, Selector};

use super::{
    search::{parse_search_html, SearchResult},
    DlsiteClient,
};
use crate::{error::Result, utils::ToParseError as _};

use self::options::CircleQuery;

/// Client to get circle-related content from DLsite.
#[derive(Clone, Debug)]
pub struct CircleClient<'a> {
    pub(crate) c: &'a DlsiteClient,
}

impl<'a> CircleClient<'a> {
    /// Search circle-related products.
    pub async fn get_circle(&self, circle_id: &str, options: &CircleQuery) -> Result<SearchResult> {
        let query_path = options.to_path(circle_id);
        let html = self.c.get(&query_path).await?;
        let html = Html::parse_fragment(&html);
        let products_html = html
            .select(&Selector::parse("#search_result_list").unwrap())
            .next()
            .to_parse_error("Product list not found")?;

        let count: i32 = html
            .select(&Selector::parse(".page_total > strong").unwrap())
            .next()
            .to_parse_error("No total item count found")?
            .text()
            .next()
            .to_parse_error("No total item count found 2")?
            .parse()
            .to_parse_error("Failed to parse total item count")?;

        let products = parse_search_html(&products_html.html())?;

        Ok(SearchResult {
            products,
            count,
            query_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::DlsiteClient;

    #[tokio::test]
    async fn get_circle_1() {
        let client = DlsiteClient::default();
        let res = client
            .circle()
            .get_circle(
                "RG24350",
                &super::CircleQuery {
                    ..Default::default()
                },
            )
            .await
            .expect("Failed to search");

        assert_eq!(50, res.products.len());

        res.products.iter().for_each(|r| {
            dbg!(&r);
        });

        let res = client
            .circle()
            .get_circle(
                "RG24350",
                &super::CircleQuery {
                    page: Some(2),
                    ..Default::default()
                },
            )
            .await
            .expect("Failed to search");

        assert!(!res.products.is_empty());
    }
}
