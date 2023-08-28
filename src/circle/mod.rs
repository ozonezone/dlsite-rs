mod options;

use scraper::{Html, Selector};

use crate::{
    search::{parse_search_html, SearchResult},
    utils::ToParseError,
    DlsiteClient, Result,
};

use self::options::CircleQueryOptions;

pub struct CircleResult {
    pub products: Vec<SearchResult>,
}

impl DlsiteClient {
    pub async fn get_circle(
        &self,
        circle_id: &str,
        options: &CircleQueryOptions,
    ) -> Result<CircleResult> {
        let html = self.get(&options.to_path(circle_id)).await?;
        let html = Html::parse_fragment(&html);
        let products_html = html
            .select(&Selector::parse("#search_result_list").unwrap())
            .next()
            .to_parse_error("Product list not found")?;

        let products = parse_search_html(&products_html.html())?;

        Ok(CircleResult { products })
    }
}

#[cfg(test)]
mod tests {
    use crate::{DlsiteClient};

    #[tokio::test]
    async fn get_circle_1() {
        let client = DlsiteClient::default();
        let res = client
            .get_circle(
                "RG24350",
                &super::CircleQueryOptions {
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
            .get_circle(
                "RG24350",
                &super::CircleQueryOptions {
                    page: Some(2),
                    ..Default::default()
                },
            )
            .await
            .expect("Failed to search");

        assert!(!res.products.is_empty());
    }
}
