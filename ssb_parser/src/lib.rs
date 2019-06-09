//! Parser component of subtitle format implementation.
//! ```
//! // Imports
//! use std::{
//!     convert::TryFrom,
//!     fs::File,
//!     io::{BufReader,Cursor},
//!     path::Path
//! };
//! use ssb_parser::data::{Ssb,SsbRender};
//! // Data
//! let ssb_reader1 = Cursor::new("...");
//! let ssb_reader2 = BufReader::new(File::open("/foo/bar.ssb").unwrap());
//! // Parsing
//! let mut ssb = Ssb::default();
//! ssb.parse(ssb_reader1, None).unwrap()
//!     .parse(ssb_reader2, Some(Path::new("/foo/"))).unwrap();
//! let ssb_render = SsbRender::try_from(ssb).unwrap();
//! // Print
//! println!("{:#?}", ssb_render);
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]


/// Minor types for data in this crate.
pub mod types;
/// Internal utility structs & functions for data processing of this crate.
mod utils;
/// Data processors and storage of this crate.
pub mod data;