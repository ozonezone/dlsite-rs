//! Common interfaces

pub mod product;
pub mod query;
pub mod genre {
    //! Interfaces related to genre.

    /// Genre struct
    #[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Genre {
        pub name: String,
        pub id: String,
    }
}
