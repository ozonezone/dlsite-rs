//! Common used interfaces.

use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::DeserializeFromStr;
use strum::{Display, EnumString};

/// Work category
#[derive(Display, EnumString, Debug, PartialEq, Clone, DeserializeFromStr, serde::Serialize)]
pub enum WorkType {
    /// Game category
    ///
    /// JP: アクション
    /// EN: Action
    ACN,
    /// JP: クイズ
    /// EN: Quiz
    QIZ,
    /// JP: アドベンチャー
    /// EN: Adventure
    ADV,
    /// JP: ロールプレイング
    /// EN: Role-playing
    RPG,
    /// JP: テーブル
    /// EN: Table
    TBL,
    /// JP: デジタルノベル
    /// EN: Digital Novel
    DNV,
    /// JP: シミュレーション
    /// EN: Simulation
    SLN,
    /// JP: タイピング
    /// EN: Typing
    TYP,
    /// JP: シューティング
    /// EN: Shooting
    STG,
    /// JP: パズル
    /// EN: Puzzle
    PZL,
    /// JP: その他ゲーム
    /// EN: Miscellaneous Games
    ETC,

    /// Mange category
    ///
    /// JP: マンガ
    /// EN: Manga
    MNG,
    /// JP: 劇画
    /// EN: Gekiga
    SCM,
    /// JP: WEBTOON
    /// EN: Webtoon
    WBT,

    /// CG + Illustrations category
    ///
    /// JP: CG・イラスト
    /// EN: CG + Illustrations
    ICG,

    // Novel category
    //
    /// JP: ノベル
    /// EN: Novel
    NRE,
    /// JP: 官能小説
    /// EN: Erotic Novel
    KSV,

    /// Video category
    ///
    /// JP: 動画
    /// EN: Video
    MOV,

    /// Voice / ASMR category
    ///
    /// JP: ボイス・ASMR
    /// EN: Voice / ASMR
    SOU,

    /// Music category
    ///
    /// JP: 音楽
    /// EN: Music
    MUS,

    /// Tools / Accessories category
    ///
    /// JP: ツール/アクセサリ
    /// EN: Tools / Accessories
    TOL,
    /// JP: 画像素材
    /// EN: Illustration Materials
    IMT,
    /// JP: 音素材
    /// EN: Music Materials
    AMT,

    /// Miscellaneous category
    ///
    /// JP: その他
    /// EN: Miscellaneous
    ET3,
    /// JP: ボイスコミック
    /// EN: Voiced Comics
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
#[derive(Display, EnumString, PartialEq, DeserializeFromStr, Debug, Clone)]
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
#[derive(Display, EnumString, PartialEq, Debug, Clone, DeserializeFromStr)]
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
