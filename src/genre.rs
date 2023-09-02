/// Genre struct
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Genre {
    pub name: String,
    pub id: String,
}
