<p align="center">
    <img src="https://substation-beta.github.io/assets/img/logo.png" alt="Logo"/>
</p>

Crates | [![Crate Version](https://img.shields.io/crates/v/ssb_parser.svg?label=ssb_parser&logo=rust)](https://crates.io/crates/ssb_parser) [![Crate Version](https://img.shields.io/crates/v/ssb_renderer.svg?label=ssb_renderer&logo=rust)](https://crates.io/crates/ssb_renderer) [![Crate Version](https://img.shields.io/crates/v/ssb_filter.svg?label=ssb_filter&logo=rust)](https://crates.io/crates/ssb_filter)
:---|:---
Documentation | [![Crate Docs Version](https://img.shields.io/crates/v/ssb_parser.svg?label=ssb_parser&logo=rust&color=informational)](https://docs.rs/ssb_parser) [![Crate Docs Version](https://img.shields.io/crates/v/ssb_renderer.svg?label=ssb_renderer&logo=rust&color=informational)](https://docs.rs/ssb_renderer) [![Crate Docs Version](https://img.shields.io/crates/v/ssb_filter.svg?label=ssb_filter&logo=rust&color=informational)](https://docs.rs/ssb_filter)
Code quality | [![Build Status](https://img.shields.io/travis/substation-beta/ssb_implementation.svg?logo=travis)](https://travis-ci.org/substation-beta/ssb_implementation) [![Build Status](https://img.shields.io/appveyor/ci/Youka/ssb-implementation.svg?logo=appveyor)](https://ci.appveyor.com/project/Youka/ssb-implementation) [![Code Coverage](https://img.shields.io/codecov/c/github/substation-beta/ssb_implementation.svg?logo=Codecov)](https://codecov.io/gh/substation-beta/ssb_implementation) [![dependency status](https://deps.rs/repo/github/substation-beta/ssb_implementation/status.svg)](https://deps.rs/repo/github/substation-beta/ssb_implementation)
Properties | [![License](https://img.shields.io/github/license/substation-beta/ssb_implementation.svg?logo=github)](https://github.com/substation-beta/ssb_implementation/blob/master/LICENSE) [![Minimal rust version](https://img.shields.io/badge/rust-v1.37%2B-blue?logo=rust)](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1370-2019-08-15)  [![Last commit](https://img.shields.io/github/last-commit/substation-beta/ssb_implementation.svg?logo=github)](https://github.com/substation-beta/ssb_implementation/graphs/commit-activity)
Platforms | [![Windows support](https://img.shields.io/badge/Windows-supported-success.svg?logo=Windows)](https://en.wikipedia.org/wiki/Microsoft_Windows) [![Linux support](https://img.shields.io/badge/Linux-supported-success.svg?logo=Linux)](https://en.wikipedia.org/wiki/Linux) [![Mac support](https://img.shields.io/badge/OSX-not%20willingly-inactive.svg?logo=Apple)](https://en.wikipedia.org/wiki/MacOS)
Contact | [![Discord channel](https://img.shields.io/discord/586927398277087235.svg?logo=discord)](https://discord.gg/H8HnPSv) [![Github issues](https://img.shields.io/github/issues/substation-beta/ssb_implementation.svg?logo=github)](https://github.com/substation-beta/ssb_implementation/issues)

&nbsp;

---

| Index of contents |
|:---:|
| [Substation Beta](#substation-beta) &bull; [Components](#components) &bull; [Getting started](#getting-started) &bull; [Building](#building) &bull; [Contributing](#contributing) &bull; [License](#license) &bull; [Acknowledgment](#acknowledgment) |

# SubStation Beta
This project is the reference implementation of subtitle format `substation beta` (short **ssb**).

Components target desktop application development and evolve with continuation of [ssb_book](https://github.com/substation-beta/ssb_book).

# Components
Project contents consist of multiple components which build on top of each other:

**ssb_parser** &rarr; **ssb_renderer** &rarr; **ssb_filter**

## ssb_parser
Parser of text in ssb format.

* **Reads** from file or byte stream
* **Validates** content
* **Packs** data into ordered structures
* Allows **serialization** in other format (like JSON)
* Relevant for **rust developers**

See sub-project [ssb_parser](https://github.com/substation-beta/ssb_implementation/tree/master/ssb_parser).

## ssb_renderer
2d graphics software renderer for ssb format.

* Builds upon **ssb_parser** for input processing
* **Renders** 2-dimensional graphics on system memory buffers
* **High-performance** by efficient hardware workload
* Relevant for **rust developers**

See sub-project [ssb_renderer](https://github.com/substation-beta/ssb_implementation/tree/master/ssb_renderer).

## ssb_filter
Interfaces to ssb rendering for video frameserving and language wrapping.

* Builds upon **ssb_renderer** for graphics rendering (including **ssb_parser** for input processing)
* **Plugin** binary for immediate use in popular frameservers
* **C API** provides access by [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface)
* Relevant for **c developers** and **frameserver users**

See sub-project [ssb_filter](https://github.com/substation-beta/ssb_implementation/tree/master/ssb_filter).

# Getting started
*TODO*
## Install
*TODO*
## First steps
*TODO*
## Documentation
*TODO*

# Building
*TODO*

# Contributing
We welcome contributers but insist on working by our rules. The principle **quality > quantity** has to be followed through every part of this project.

For details, see [Contributing](https://github.com/substation-beta/ssb_implementation/blob/master/CONTRIBUTING.md).

# License
This project and all of its components are licensed under **Apache-2.0**. Distributed on an "AS-IS" basis, there's no warranty, a limited liability and no grant of trademark rights.

For more, see [License](https://github.com/substation-beta/ssb_implementation/blob/master/LICENSE).

# Acknowledgment
* [ASS (Advanced Substation Alpha)](https://en.wikipedia.org/wiki/SubStation_Alpha#Advanced_SubStation_Alpha)