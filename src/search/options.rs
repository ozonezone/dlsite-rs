//! Search options for dlsite product search

use strum::Display;

use crate::{
    interface::{FileType, WorkCategory, WorkType},
    search::macros::*,
};

// Struct that can be converted dlsite url (below is example). All params are optional.
// https://www.dlsite.com/maniax/fsr/=
// /language/jp
// /sex_category%5B0%5D/male
// /keyword/a
// /regist_date_end/2022-08-25
// /price_low/801
// /price_high/1000
// /ana_flg/on
// /age_category%5B0%5D/r15
// /work_category%5B0%5D/doujin
// /order%5B0%5D/trend
// /work_type_category%5B0%5D/audio
// /work_type_category_name%5B0%5D/%E3%83%9C%E3%82%A4%E3%82%B9%E3%83%BBASMR
// /genre%5B0%5D/497
// /genre_name%5B0%5D/ASMR
// /options_and_or/and
// /options%5B0%5D/JPN/options%5B1%5D/NM
// /options_not%5B0%5D/AIG/options_not%5B1%5D/AIP
// /options_name%5B0%5D/%E6%97%A5%E6%9C%AC%E8%AA%9E%E4%BD%9C%E5%93%81/options_name%5B1%5D/%E8%A8%80%E8%AA%9E%E4%B8%8D%E5%95%8F%E4%BD%9C%E5%93%81
// /rate_average%5B0%5D/2
// /per_page/30
// /page/1
// /campaign/campaign
// /soon/1
// /dlsite_only/1
// /is_pointup/1
// /is_free/1
// /release_term/old
// /price_category/4
// /show_type/1
// /from/fs.detail

#[derive(Default)]
pub struct ProductSearchOptions {
    /// Display lang
    pub language: Language,
    pub keyword_creator: Option<String>,
    pub sex_category: Option<Vec<SexCategory>>,
    pub keyword: Option<String>,
    pub regist_date_end: Option<String>,
    pub price_low: Option<u32>,
    pub price_high: Option<u32>,
    /// Sales status
    pub ana_flg: Option<AnaFlg>,
    pub age_category: Option<Vec<crate::interface::AgeCategory>>,
    pub work_category: Option<Vec<WorkCategory>>,
    pub order: Option<Order>,
    pub work_type: Option<Vec<WorkType>>,
    pub work_type_category: Option<Vec<WorkTypeCategory>>,
    pub genre: Option<Vec<u32>>,
    pub options_and_or: Option<OptionAndOr>,
    pub options: Option<Vec<String>>,
    pub options_not: Option<Vec<String>>,
    pub file_type: Option<Vec<FileType>>,
    pub rate_average: Option<u32>,
    /// 30, 50 or 100
    pub per_page: Option<u32>,
    pub page: Option<u32>,
    pub campagin: Option<bool>,
    /// Whether the sales end date is in 24 hours
    pub soon: Option<bool>,
    pub is_pointup: Option<bool>,
    pub is_free: Option<bool>,
    pub release_term: Option<ReleaseTerm>,
}

impl ProductSearchOptions {
    pub(super) fn to_path(&self) -> String {
        let mut path = "/fsr/ajax/=".to_string();

        push!(path, self, language);
        push_option!(path, self, keyword_creator);
        push_option_array!(path, self, sex_category);
        push_option!(path, self, keyword);
        push_option!(path, self, regist_date_end);
        push_option!(path, self, price_low);
        push_option!(path, self, price_high);
        push_option!(path, self, ana_flg);
        push_option_array!(path, self, age_category);
        push_option_array!(path, self, work_category);
        push_option!(path, self, order);
        push_option_array!(path, self, work_type);
        push_option_array!(path, self, work_type_category);
        push_option_array!(path, self, genre);
        push_option!(path, self, options_and_or);
        push_option_array!(path, self, options);
        push_option_array!(path, self, options_not);
        push_option_array!(path, self, file_type);
        push_option!(path, self, rate_average);
        push_option!(path, self, per_page);
        push_option!(path, self, page);
        push_option_bool!(path, self, campagin);
        push_option_bool!(path, self, soon);
        push_option_bool!(path, self, is_pointup);
        push_option_bool!(path, self, is_free);
        push_option!(path, self, release_term);

        path
    }
}

#[derive(Display, Default)]
#[strum(serialize_all = "snake_case")]
pub enum Language {
    #[default]
    Jp,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum SexCategory {
    Male,
    Female,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum AnaFlg {
    Off,
    On,
    Reserve,
    All,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum Order {
    Trend,
    /// 新しい
    Release,
    /// 古い
    ReleaseD,
    /// DL数が多い
    DlD,
    /// DL数が少ない
    Dl,
    /// 安い
    Price,
    /// 高い
    PriceD,
    /// 評価が高い
    RateD,
    /// レビューが多い
    ReviewD,
}

/// 作品形式(親カテゴリ)
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum WorkTypeCategory {
    Game,
    Comic,
    Illust,
    Novel,
    MovieAudio,
    Music,
    Tool,
    Etc,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum OptionAndOr {
    And,
    Or,
}

#[derive(Display)]
pub enum ReleaseTerm {
    None,
    Week,
    Month,
    Year,
    Old,
}

#[cfg(test)]
mod tests {
    use crate::search::options::*;

    #[test]
    fn product_search_param_default() {
        assert_eq!(
            "/fsr/ajax/=/language/jp",
            super::ProductSearchOptions::default().to_path()
        );
    }
    #[test]
    fn product_search_param_1() {
        assert_eq!(
            "/fsr/ajax/=/language/jp/sex_category[0]/male/price_low/801/file_type[0]/PNG/file_type[1]/EXE/soon/1",
            super::ProductSearchOptions {
                sex_category: Some(vec![SexCategory::Male]),
                price_low: Some(801),
                file_type: Some(vec![FileType::PNG, FileType::EXE]),
                soon: Some(true),
                is_free: Some(false),
                ..Default::default()
            }
            .to_path()
        );
    }
}
