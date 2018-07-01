// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Compiles Windows resource files (.rc) into a Rust program.
//!
//! This crate provides utilities for compiling .rc files into object files when targeting Windows.
//! .rc files specify icons, version information, and *localisable resources* such as menu strings,
//! which are embedded into a binary at link time. Currently, Rust does not natively support .rc
//! files, so this crate must be used instead to achieve the same effect.

#![cfg(windows)]
#![forbid(warnings)]
#![deny(unused)]
#![forbid(box_pointers)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unused_extern_crates)]
#![forbid(unused_import_braces)]
#![deny(unused_qualifications)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]
#![cfg_attr(feature = "cargo-clippy", forbid(clippy))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]
#![cfg_attr(feature = "cargo-clippy", forbid(clippy_cargo))]
#![cfg_attr(feature = "cargo-clippy", forbid(clippy_complexity))]
#![cfg_attr(feature = "cargo-clippy", forbid(clippy_correctness))]
#![cfg_attr(feature = "cargo-clippy", forbid(clippy_perf))]
#![cfg_attr(feature = "cargo-clippy", forbid(clippy_style))]

#[macro_use(concat_string)]
extern crate concat_string;

use std::io;
use std::path::{Path, PathBuf};

#[cfg(target_env = "gnu")]
#[path = "gnu.rs"]
mod imp;
#[cfg(target_env = "msvc")]
#[path = "msvc.rs"]
mod imp;

#[derive(Clone, Debug)]
/// A builder for compiling Windows resources.
pub struct Build {
    /// A list of additional include paths to use during preprocessing.
    extra_include_dirs: Vec<PathBuf>,
    /// A list of additional preprocessor definitions to use during preprocessing.
    extra_cpp_defs: Vec<(String, Option<String>)>,
    /// A list of preprocessor symbols to undefine during preprocessing.
    cpp_undefs: Vec<String>,
}

impl Build {
    /// Creates a new, empty builder.
    pub fn new() -> Self {
        Self {
            extra_include_dirs: Vec::new(),
            extra_cpp_defs: Vec::new(),
            cpp_undefs: Vec::new(),
        }
    }

    /// Specifies an additional include path to use during preprocessing.
    pub fn include<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.extra_include_dirs.push(path.as_ref().to_owned());
        self
    }

    /// Specifies an additional preprocessor definition to use during preprocessing.
    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, name: &str, value: V) -> &mut Self {
        self.extra_cpp_defs
            .push((name.to_owned(), value.into().map(|s| s.to_owned())));
        self
    }

    /// Specifies a preprocessor symbol to undefine during preprocessing.
    pub fn undefine(&mut self, name: &str) -> &mut Self {
        self.cpp_undefs.push(name.to_owned());
        self
    }

    /// Compiles a Windows resource file (.rc).
    pub fn compile<P: AsRef<Path>>(&mut self, rc_file: P) -> io::Result<()> {
        Self::find_resource_compiler().and_then(|compiler| self.compile_resource(rc_file, compiler))
    }
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}
