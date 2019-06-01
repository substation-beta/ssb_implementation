//! Parser component of subtitle format implementation.
//! ```
//! let ssb_data = SsbParser::new("...").unwrap().data();
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]


/// Error types covering all bad situations in this crate.
pub mod error;
/// Minor types for data in this crate.
pub mod types;
/// Internal processing utilities of this crate.
mod utils;
/// Data processors and storage of this crate.
pub mod data;