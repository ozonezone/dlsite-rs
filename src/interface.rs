//! Common used interfaces.

use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::DeserializeFromStr;
use strum::{Display, EnumString};

/// Work category
#[derive(Display, EnumString, Debug, PartialEq, Clone, DeserializeFromStr)]
pub enum WorkType {
    /// アクション
    ACN,
    /// クイズ
    QIZ,
    ADV,
    RPG,
    TBL,
    DNV,
    SLN,
    TYP,
    STG,
    PZL,
    ETC,

    /// マンガ
    MNG,
    /// 劇画
    SCM,
    /// webtoon
    WBT,

    /// CG・イラスト
    ICG,

    // Novel
    /// ノベル
    NRE,
    /// 官能小説
    KSV,

    /// 動画
    MOV,

    /// ボイス・ASMR
    SOU,

    /// 音楽
    MUS,

    // Tool
    /// ツール
    TOL,
    /// 画像素材
    IMT,
    /// 音素材
    AMT,

    /// その他
    ET3,
    /// ボイスコミック
    VCM,

    #[strum(default)]
    Unknown(String),
}

/// Age category
#[derive(Display, Debug, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
#[strum(serialize_all = "snake_case")]
pub enum AgeCategory {
    #[serde(with = "i8")]
    General = 1,
    #[serde(with = "i8")]
    R15 = 2,
    #[serde(with = "i8")]
    Adult = 3,
}

/// Work category (parent category)
#[derive(Display, EnumString, DeserializeFromStr, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum WorkCategory {
    /// 同人
    Doujin,
    /// Adult: 成年コミック
    Books,
    /// Adult: 美少女ゲーム, General: PCソフト
    Pc,
    /// スマホゲーム
    App,

    #[strum(default)]
    Unknown(String),
}

/// File type
#[derive(Display, EnumString, Debug, Clone, DeserializeFromStr)]
pub enum FileType {
    EXE,
    HTI,
    HTE,
    HMO,
    IJP,
    IGF,
    IME,
    IBP,
    PNG,
    AVI,
    MVF,
    MPG,
    MWM,
    MP4,
    AAC,
    WAV,
    MP3,
    ADO,
    WMA,
    FLC,
    OGG,
    PDF,
    APK,
    ET1,

    #[strum(default)]
    Unknown(String),
}
