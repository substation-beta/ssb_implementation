//! Filter interfaces for various frameservers to render subtitle format.
//! ```
//! // TODO
//! ```
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]


/// C API (usable f.e. with [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface)).
pub mod c;
/// [Vapoursynth](www.vapoursynth.com) frameserver.
#[cfg(feature = "vapoursynth-interface")]
pub mod vapoursynth;