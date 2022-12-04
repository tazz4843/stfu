//! # stfu: Shut The Ferris Up
//!
//! A general word filter for whatever use you want.
//!
//! # Usage
//! ```
//! use stfu::types::OwnedFilter;
//! use stfu::word_lists::category::SEXUAL_ANATOMY_SEXUAL_ACTS;
//! # fn main() {
//! let mut filter = OwnedFilter::default();
//! filter.add_slice(&SEXUAL_ANATOMY_SEXUAL_ACTS);
//! assert_eq!(filter.filter_string("hello world"), None);
//! assert_eq!(filter.filter_string("Mary had a little fucking lamb"), Some("fucking"));
//! # }
//! ```
//!
//! ```
//! use stfu::word_lists::category::SEXUAL_ANATOMY_SEXUAL_ACTS;
//! use stfu::filter::filter_string;
//! # fn main() {
//! assert_eq!(filter_string("hello world", &SEXUAL_ANATOMY_SEXUAL_ACTS), None);
//! assert_eq!(filter_string("Mary had a little fucking lamb", &SEXUAL_ANATOMY_SEXUAL_ACTS), Some("fucking"));
//! # }

#![deny(missing_docs)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]

pub mod filter;
pub mod types;
pub mod word_lists;
