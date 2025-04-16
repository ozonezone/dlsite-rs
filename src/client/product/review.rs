use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use super::super::genre::Genre;

pub enum ReviewSortOrder {
    New,
    Top,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ProductReview {
    pub is_success: bool,
    pub error_msg: String,
    pub review_list: Vec<Review>,
    #[serde(deserialize_with = "deserialize_reviewer_genre")]
    pub reviewer_genre_list: Option<Vec<(Genre, i32)>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Review {
    pub member_review_id: String,
    pub workno: String,
    pub reviewer_id: String,
    pub status: String,
    pub recommend: String,
    pub spoiler: String,
    pub review_title: String,
    pub review_text: String,
    pub entry_date: String,
    pub regist_date: String,
    pub good_review: String,
    pub bad_review: String,
    pub circle_id: Option<String>,
    pub nick_name: Option<String>,
    pub popularity: Option<String>,
    pub rate: Option<String>,
    pub circle_name: Option<String>,
    pub top_sort_key: Option<String>,
    pub reviewer_status: String,
    pub is_purchased: String,
    pub rate_num: String,
    pub reviewer_rank: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_genre")]
    pub genre: Vec<Genre>,
}

fn deserialize_genre<'de, D>(deserializer: D) -> std::result::Result<Vec<Genre>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<HashMap<String, String>> = Deserialize::deserialize(deserializer)?;

    if let Some(s) = s {
        let mut genres = vec![];
        for (key, value) in s {
            genres.push(Genre {
                name: value,
                id: key,
            });
        }
        Ok(genres)
    } else {
        Ok(vec![])
    }
}

fn deserialize_reviewer_genre<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Vec<(Genre, i32)>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ReviewerGenre {
        genre: String,
        genre_count: String,
        name: String,
    }
    let s: Option<Vec<ReviewerGenre>> = Deserialize::deserialize(deserializer)?;

    if let Some(s) = s {
        let mut genres = vec![];
        for reviewer_genre in s {
            genres.push((
                Genre {
                    name: reviewer_genre.name,
                    id: reviewer_genre.genre,
                },
                reviewer_genre.genre_count.parse().unwrap_or(0),
            ))
        }
        Ok(Some(genres))
    } else {
        Ok(None)
    }
}
