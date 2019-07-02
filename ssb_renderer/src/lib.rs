//! Renderer component of subtitle format implementation.
//! ```
//! // TODO
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]

/// Re-exports (with interfaces required by public users).
pub use ssb_parser;

/// Rendering backend for general 2D graphics.
pub mod g2d;
/// Supportive types for rendering.
pub mod types;
/// High-level rendering interface.
pub mod rendering;