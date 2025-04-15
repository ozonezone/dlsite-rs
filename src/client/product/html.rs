use std::collections::HashMap;

use chrono::NaiveDate;
use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::{
    client::common::AgeCategory, client::genre::Genre, error::Result, utils::ToParseError,
    DlsiteError,
};

use super::ProductPeople;

/// Product data got from html
#[derive(Debug)]
pub struct ProductHtml {
    pub released_at: NaiveDate,
    pub age_rating: Option<AgeCategory>,
    pub circle_id: String,
    pub circle_name: String,
    pub images: Vec<String>,
    pub people: ProductPeople,
    pub genre: Vec<Genre>,
    pub series: Option<String>,
    pub file_format: Vec<String>,
    pub file_size: Option<String>,
    pub product_format: Vec<String>,
    pub description_html: Option<String>,
    pub event: Vec<String>,
    pub pages: Option<String>,
    pub update_information: Option<String>,
    pub scenario: Vec<String>,
    pub illustration: Vec<String>,
    pub misc: Vec<String>,
    pub langs: Vec<String>,
    pub music: Vec<String>,
    pub sys_req: Option<String>,
    pub coupling: Vec<String>,
    pub lang_refs: Vec<(String, String)>,
}

pub(super) fn parse_product_html(html: &Html) -> Result<ProductHtml> {
    let circle = html
        .select(&Selector::parse("#work_maker .maker_name a").unwrap())
        .next()
        .to_parse_error("No circle found")?;
    let circle_name = circle
        .text()
        .next()
        .to_parse_error("No circle name found")?
        .to_string();
    let circle_id = circle
        .value()
        .attr("href")
        .to_parse_error("No circle id found")?
        .split('/')
        .next_back()
        .to_parse_error("Failed to parse circle id")?
        .split('.')
        .next()
        .to_parse_error("Failed to parse circle id")?
        .to_string();

    let images: Vec<String> = html
        .select(&Selector::parse(".product-slider-data > div").unwrap())
        .flat_map(|element| {
            let url = element.value().attr("data-src")?;
            let url: Url = format!("https:{}", url).parse().ok()?;
            Some(url.to_string())
        })
        .collect();

    // work_outline_table
    let mut work_outline_table = get_work_outline_table(html);
    work_outline_table.remove("作者");
    work_outline_table.remove("声優");
    let file_size = work_outline_table
        .remove("ファイル容量")
        .and_then(|v| {
            v.select(&Selector::parse("div").unwrap()).next().map(|v| {
                Some(
                    v.text()
                        .collect::<String>()
                        .trim()
                        .strip_prefix("総計\u{a0}")?
                        .to_owned(),
                )
            })
        })
        .flatten();
    let work_genre_extractor = |work_outline_table: &mut HashMap<String, ElementRef>, key: &str| {
        work_outline_table
            .remove(key)
            .and_then(|v| {
                v.select(&Selector::parse(".work_genre").unwrap())
                    .next()
                    .map(|v| {
                        v.child_elements()
                            .map(|v| v.text().collect::<String>().trim().to_owned())
                            .collect::<Vec<_>>()
                    })
            })
            .unwrap_or_default()
    };
    let a_extractor = |work_outline_table: &mut HashMap<String, ElementRef>, key: &str| {
        work_outline_table
            .remove(key)
            .map(|v| {
                v.select(&Selector::parse("a").unwrap())
                    .map(|v| v.text().collect::<String>().trim().to_owned())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    };
    let product_format = work_genre_extractor(&mut work_outline_table, "作品形式");
    let description_html: Option<String> = html
        .select(&Selector::parse("[itemprop='description']").unwrap())
        .next()
        .map(|v| v.inner_html());
    let event = a_extractor(&mut work_outline_table, "イベント");
    let pages = work_outline_table
        .remove("ページ数")
        .map(|v| v.text().collect::<String>());
    let update_information = work_outline_table
        .remove("更新情報")
        .map(|v| v.text().collect::<String>().trim().to_owned());
    let scenario = a_extractor(&mut work_outline_table, "シナリオ");
    let illustration = a_extractor(&mut work_outline_table, "イラスト");
    let misc = work_genre_extractor(&mut work_outline_table, "その他");
    let langs = work_genre_extractor(&mut work_outline_table, "対応言語");
    let lang_refs = html
        .select(&Selector::parse(".work_edition .type_trans > a").unwrap())
        .filter_map(|v| {
            Some((
                v.text().collect::<String>().trim().to_owned(),
                v.attr("href")?.to_owned(),
            ))
        })
        .collect::<Vec<_>>();
    let music = a_extractor(&mut work_outline_table, "音楽");
    let sys_req = work_outline_table
        .remove("動作環境")
        .map(|v| v.text().collect::<String>().trim().to_owned());
    let coupling = a_extractor(&mut work_outline_table, "カップリング");
    let file_format = work_outline_table
        .remove("ファイル形式")
        .and_then(|v| {
            v.select(&Selector::parse(".work_genre").unwrap())
                .next()
                .map(|v| {
                    v.child_elements()
                        .map(|v| v.text().collect::<String>())
                        .map(|v| {
                            let v = v.trim();
                            v.strip_prefix("/").unwrap_or(v).trim().to_owned()
                        })
                        .collect::<Vec<_>>()
                })
        })
        .unwrap_or_default();
    let age_rating = work_outline_table
        .remove("年齢指定")
        .map(|v| {
            Ok::<_, DlsiteError>(
                v.select(&Selector::parse("span").unwrap())
                    .next()
                    .to_parse_error("No age rating found")?
                    .inner_html(),
            )
        })
        .transpose()?;
    let age_rating = match age_rating {
        Some(age_rating) => Some(match age_rating.as_str() {
            "全年齢" => AgeCategory::General,
            "R18" => AgeCategory::Adult,
            "R-15" => AgeCategory::R15,
            _ => {
                return Err(DlsiteError::ParseError(format!(
                    "failed to convert {age_rating} to enum"
                )))
            }
        }),
        None => None,
    };

    let series = work_outline_table.remove("シリーズ名");
    let series = series.map(|series| series.text().collect::<String>().trim().to_owned());

    let released_at = work_outline_table
        .remove("販売日")
        .to_parse_error("No released_at found")?
        .text()
        .next()
        .to_parse_error("No released_at found")?;
    let released_at = regex::Regex::new(r"\d*年\d*月\d*日")
        .unwrap()
        .find(released_at)
        .to_parse_error("Failed to parse released_at")?
        .as_str();
    let released_at = NaiveDate::parse_from_str(released_at, "%Y年%m月%d日")
        .map_err(|_| DlsiteError::ParseError("Failed to parse released_at".to_string()))?;
    let genre = work_outline_table
        .remove("ジャンル")
        .map(|element| {
            element
                .select(&Selector::parse("a").unwrap())
                .filter_map(|element| {
                    let name = element.text().next()?.to_string();
                    let mut id = None;
                    let mut next = false;
                    element.value().attr("href")?.split('/').for_each(|s| {
                        if next {
                            id = Some(s.to_string());
                            next = false;
                        }
                        if s == "genre" {
                            next = true;
                        }
                    });
                    id.map(|id| Genre { name, id })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if !work_outline_table.is_empty() {
        return Err(DlsiteError::ParseError(format!(
            "failed to parse tags {:?}",
            work_outline_table.len()
        )));
    }
    Ok(ProductHtml {
        released_at,
        age_rating,
        circle_id,
        circle_name,
        images,
        people: parse_product_people(html)?,
        genre,
        series,
        file_format,
        file_size,
        product_format,
        description_html,
        event,
        pages,
        update_information,
        scenario,
        illustration,
        misc,
        langs,
        music,
        sys_req,
        coupling,
        lang_refs,
    })
}

pub(super) fn parse_product_people(html: &Html) -> Result<ProductPeople> {
    let work_outline_table = get_work_outline_table(html);

    macro_rules! get_people {
        ($key:literal) => {
            work_outline_table
                .get($key)
                .map(|element| {
                    element
                        .select(&Selector::parse("a").unwrap())
                        .filter_map(|element| element.text().next().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .filter(|v| !v.is_empty())
        };
    }

    Ok(ProductPeople {
        author: get_people!("作者"),
        scenario: get_people!("シナリオ"),
        illustrator: get_people!("イラスト"),
        voice_actor: get_people!("声優"),
    })
}

fn get_work_outline_table(html: &Html) -> HashMap<String, ElementRef> {
    let mut map = HashMap::new();
    for element in html.select(&Selector::parse("#work_outline tr").unwrap()) {
        let th = element.select(&Selector::parse("th").unwrap()).next();
        let td = element.select(&Selector::parse("td").unwrap()).next();
        if let (Some(th), Some(td)) = (th, td) {
            let th = th.text().next();
            if let Some(th) = th {
                let th = th.trim().to_string();
                map.insert(th, td);
            }
        }
    }
    map
}
