//! Renderer component of subtitle format implementation.
//! ```
//! // TODO
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]

// Rendering backend for general 2D graphics.
mod g2d;
pub use g2d::raster::image;
// Rendering frontend for SSB output.
mod ssb;
pub use ssb::{error::RenderingError,rendering::*};

// Re-exports (interfaces required by public users).
pub use ssb_parser;