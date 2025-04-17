# dlsite-rs

This is a library to get information about products on DLsite. Some information
is not available on the HTML page, so this library also makes requests to the
AJAX API.

## NOTE

- This library is still wip, and the API may change.
- Only the parts I needed are implemented, so there are many unimplemented
  parts. PR is welcome.
- Especially in the JSON API, the DLsite side may change the specification.
  Expect breaking changes due to such changes.

## Implemented features

- [ ] Get product information by scraping html and using ajax api for web.
  - [x] Basic information
  - [ ] Additional information
  - [ ] Multi-language support (Currently, this crate uses Japanese page to
        parse html)
- [x] Get product review
- [x] Get product information using api.
- [x] Search product
- [ ] Get circle info
  - [x] Get circle product list
  - [ ] Get circle sale list
- [ ] Login and user related feature
- [ ] Get ranking

## Example

- Get product by api

  ```rust
  use dlsite::DlsiteClient;

  #[tokio::main]
  async fn main() {
      let client = DlsiteClient::default();
      let product = client.product_api().get("RJ01014447").await.unwrap();
      assert_eq!(product.creators.unwrap().voice_by.unwrap()[0].name, "佐倉綾音");
  }
  ```

- Search products

  ```rust
  use dlsite::{DlsiteClient, client::search::SearchProductQuery, interface::query::*};

  #[tokio::main]
  async fn main() {
      let client = DlsiteClient::default();
      let product = client
          .search()
          .search_product(&SearchProductQuery {
              sex_category: Some(vec![SexCategory::Male]),
              keyword: Some("ASMR".to_string()),
              ..Default::default()
          })
          .await
          .expect("Failed to search");
      dbg!(&product);
  }
  ```
