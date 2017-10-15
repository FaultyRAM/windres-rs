// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! windres-rs test build script.

#![cfg(windows)]

extern crate windres;

use windres::Build;

/// Build script entry point.
fn main() {
    Build::new().compile("windres-test.rc").unwrap();
}
