# dlsite-rs

This is a library to get information about products on DLsite. Some information
is not available on the HTML page, so this library also makes requests to the
AJAX API.

NOTE: This library is still wip, and the API may change. Also, only the parts I
needed are implemented, so there are many unimplemented parts.

## Features

- [ ] Get product information by scraping html and using ajax api for web.
  - [x] Basic information
  - [ ] Additional information
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
use dlsite::{DlsiteClient, product::Product};
use tokio;
#[tokio::main]
async fn main() {
    let client = DlsiteClient::default();
    let product = client.get_product_api("RJ01014447").await.unwrap();
    assert_eq!(product.creators.unwrap().voice_by.unwrap()[0].name, "佐倉綾音");
}
```

- Search products

```rust
use dlsite::{DlsiteClient, product::Product, search::options::*};
use tokio;
#[tokio::main]
async fn main() {
    let client = DlsiteClient::default();
    let product = client
        .search_product(&ProductSearchOptions {
            sex_category: Some(vec![SexCategory::Male]),
            keyword: Some("ASMR".to_string()),
            ..Default::default()
        })
        .await
        .expect("Failed to search");
    dbg!(&product);
}
```
