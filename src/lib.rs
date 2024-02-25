//! `url-builder` with a fairly simple build syntax.
//!
//! **The existence of this crate is very much a band-aid until [`url`](https://docs.rs/url/latest/url/) crate release a version
//! with a builder syntax. Since there is a open feature [request](https://github.com/servo/rust-url/issues/835) I believe it will eventually
//! happen. Which in case this crate will be deprecated.**
//!
//! # Example
//!
//! ```rust,no_run
//! use url_builder::{
//!     UrlBuilder,
//!     Part,
//!     Error,
//!     Url,
//! };
//!
//! fn main() {
//!     let mut builder = UrlBuilder::default();
//!     builder
//!         .set(Part::Scheme("http"))
//!         .set(Part::Host("192.168.0.1"))
//!         .set(Part::Port(3000));
//!     let url: Result<Url, Error> = builder.try_build();
//! }
//! ```
//!
//! There is also a [macro](url_builder) to make it easier.
//!
//! # Feature flags
//!
//! Name | Description | Default?
//! ---|---|---
//! `macros` | Enable the macro | No

#[cfg(feature="macros")]
mod r#macro;

mod models;
mod error;

pub use models::UrlBuilder;
pub use models::Part;
pub use error::Error;
pub use url::Url;

pub(crate) const SCHEME_AFFIX: &str = "://";

