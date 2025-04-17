use strum::Display;

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

/// Flag to represent sales status
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
