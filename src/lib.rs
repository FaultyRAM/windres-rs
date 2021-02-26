// Copyright (c) 2017-2021 FaultyRAM
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

#![cfg(target_os = "windows")]
#![deny(
    clippy::all,
    clippy::pedantic,
    warnings,
    future_incompatible,
    rust_2018_idioms,
    rustdoc,
    unused,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes,
    unused_results
)]
#![allow(clippy::must_use_candidate, missing_doc_code_examples)]

#[macro_use(concat_string)]
extern crate concat_string;
#[cfg(target_env = "msvc")]
extern crate find_winsdk;

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
            .push((name.to_owned(), value.into().map(ToOwned::to_owned)));
        self
    }

    /// Specifies a preprocessor symbol to undefine during preprocessing.
    pub fn undefine(&mut self, name: &str) -> &mut Self {
        self.cpp_undefs.push(name.to_owned());
        self
    }

    /// Compiles a Windows resource file (.rc).
    ///
    /// # Errors
    ///
    /// This method returns a `std::io::Error` if it either cannot locate a resource compiler or
    /// fails to compile the resource.
    pub fn compile<P: AsRef<Path>>(&mut self, rc_file: P) -> io::Result<()> {
        Self::find_resource_compiler().and_then(|compiler| self.compile_resource(rc_file, compiler))
    }
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}
