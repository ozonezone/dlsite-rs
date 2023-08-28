use anyhow::Context;
use rand::Rng;

use super::interface::Genre;
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
        "【イヤーキャンドル】道草屋-なつな3-たぬさんこんにちは【ずぶ濡れシャンプー】".to_string()
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
        println!("Testing for {}", id);
        let client = DlsiteClient::default();
        client.get_product_api(id).await.unwrap();
    }
}

#[test_case("RJ"; "rj")]
// #[test_case("VJ"; "vj")]
#[tokio::test]
async fn get_product_api_rand(product_type: &str) {
    let mut rng = rand::thread_rng();
    let mut i = 0;
    loop {
        let id = rng.gen_range(100000..1000000);
        let id = if id >= 1000000 {
            format!("{}0{}", product_type, id)
        } else {
            format!("{}{}", product_type, id)
        };
        let client = DlsiteClient::default();
        let pre_req = client
            .get(&format!("/api/=/product.json?workno={}", id))
            .await
            .unwrap();
        if pre_req.trim() == "[]" {
            println!("Testing for {}: Invalid id", id);
            continue;
        }
        println!("Testing for {}", id);
        client.get_product_api(&id).await.unwrap();

        i += 1;
        if i >= 5 {
            break;
        }
    }
}
