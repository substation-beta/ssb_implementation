//! Filter interfaces for various frameservers to render subtitle format.
//! ```
//! // TODO
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]

/// [Vapoursynth](www.vapoursynth.com) frameserver.
#[cfg(feature = "vapoursynth-interface")]
pub mod vapoursynth;