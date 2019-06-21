//! Renderer component of subtitle format implementation.
//! ```
//! // TODO
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]

// Rendering backend for general 2D graphics
mod g2d;
/// Error types for rendering process.
pub mod error;
/// High-level rendering interface
pub mod rendering;