/*!
Renderer component of subtitle format implementation.

```
// TODO
```
*/
#![doc(
    html_logo_url = "https://substation-beta.github.io/assets/img/logo.png",
    html_favicon_url  = "https://substation-beta.github.io/assets/img/logo.png",
    html_root_url = "https://substation-beta.github.io"
)]

// Project modules
mod error;
mod rendering;

// Exports
pub use crate::{error::RenderingError, rendering::*};

// Re-exports (interfaces required by public users).
pub use puny2d::raster::image;
pub use ssb_parser;