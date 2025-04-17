#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! # Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

pub mod client;
pub mod error;
pub mod interface;
mod utils;

pub use client::DlsiteClient;
pub use error::DlsiteError;
