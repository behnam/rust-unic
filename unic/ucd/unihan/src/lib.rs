// Copyright 2012-2015 The Rust Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![forbid(future_incompatible, missing_debug_implementations, unconditional_recursion, unsafe_code)]
#![deny(bad_style, unsafe_code, unused)]

extern crate unic_ucd_version;
extern crate unic_char_property;

mod readings;
pub use readings::{definition_of, mandarin_of};

use unic_ucd_version::UnicodeVersion;

mod pkg_info;
pub use pkg_info::{PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

/// The [Unicode version](https://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("../tables/unicode_version.rsv");