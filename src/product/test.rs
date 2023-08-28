use anyhow::Context;
use chrono::NaiveDate;
use test_case::test_case;

use crate::{
    genre::Genre,
    interface::{AgeCategory, WorkType},
    DlsiteClient,
};

#[tokio::test]
async fn get_product_1_content() {
    let client = DlsiteClient::default();
    let res = client.get_product("RJ403038").await.unwrap();

    assert_eq!(res.id, "RJ403038".to_string());
    assert_eq!(
        res.title,
        "【ブルーアーカイブ】ユウカASMR～頑張るあなたのすぐそばに～".to_string()
    );
    assert_eq!(res.circle_name, "Yostar");
    assert_eq!(res.circle_id, "RG62982");
    assert_eq!(res.work_type, WorkType::SOU);
    assert_eq!(
        res.released_at,
        NaiveDate::from_ymd_opt(2022, 7, 17).unwrap()
    );
    assert_eq!(res.age_rating, AgeCategory::General);
    assert_eq!(res.people.voice_actor, Some(vec!["春花らん".to_string()]));
    assert!(res.sale_count.unwrap() > 50000);
    assert!(res.genre.contains(&Genre {
        name: "ASMR".to_string(),
        id: "497".to_string()
    }));

    dbg!(&res);
}

#[tokio::test]
async fn get_product_2() {
    let client = DlsiteClient::default();
    let res = client
        .get_product("RJ01017217")
        .await
        .context("Failed to get product info");
    let res = res.unwrap();
    assert_eq!(res.id, "RJ01017217".to_string());
    assert_eq!(
        res.title,
        "【イヤーキャンドル】道草屋-なつな3-たぬさんこんにちは【ずぶ濡れシャンプー】".to_string()
    );
    assert_eq!(res.circle_name, "桃色CODE");
    assert_eq!(res.circle_id, "RG24350");

    assert_eq!(res.work_type, WorkType::SOU);
    assert_eq!(
        res.released_at,
        NaiveDate::from_ymd_opt(2023, 1, 21).unwrap()
    );
    assert_eq!(res.age_rating, AgeCategory::Adult);
    assert_eq!(
        res.people.voice_actor,
        Some(vec!["丹羽うさぎ".to_string(), "藤堂れんげ".to_string()])
    );
    assert_eq!(res.people.author, Some(vec!["桃鳥".to_string()]));
    assert!(res.sale_count.unwrap() > 10000);
    assert!(res.genre.contains(&Genre {
        name: "ASMR".to_string(),
        id: "497".to_string()
    }));

    dbg!(&res);
}

#[test_case("RJ01084246"; "otome")]
#[test_case("VJ01000513"; "soft")]
#[tokio::test]
async fn get_product_success(id: &str) {
    let client = DlsiteClient::default();
    client.get_product(id).await.unwrap();
}
