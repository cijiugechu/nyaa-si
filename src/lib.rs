#![doc = include_str!("../README.md")]

/// common traits for category
pub mod category;
/// common traits for api client
pub mod client;
/// nyaa errors
pub mod error;
mod extractor;
/// type definitions for api response
pub mod model;
/// nyaa-specific query and client
pub mod nyaa;
/// query params
pub mod query;
/// sukebei-specific query and client
pub mod sukebei;

pub use crate::client::Client;
pub use crate::nyaa::{NyaaCategory, NyaaClient};
pub use crate::query::{Query, QueryBuilder, Sort, SortOrder};
pub use crate::sukebei::{SukebeiCategory, SukebeiClient};

// re-exports:
pub use chrono;
