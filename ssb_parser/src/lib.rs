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
/// Data containers for all sort of information of this crate.
pub mod data;
/// Processors to generate & handle data of this crate.
pub mod processing;