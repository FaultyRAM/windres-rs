// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! GNU implementation details.

use std::env;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use Build;

impl Build {
    /// Locates the tool used to compile resources.
    pub(crate) fn find_resource_compiler() -> io::Result<PathBuf> {
        if let Some(p) = env::var_os("PATH").and_then(|path| {
            env::split_paths(&path)
                .map(|p| p.join("windres.exe"))
                .find(|p| p.exists())
        }) {
            Ok(p)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "could not locate windres.exe",
            ))
        }
    }

    /// Invokes the resource compiler using the current arguments.
    pub(crate) fn compile_resource<P: AsRef<Path>>(
        &self,
        rc_file: P,
        compiler: PathBuf,
    ) -> io::Result<()> {
        let mut cmd = Command::new(compiler);
        // User-specific options.
        for inc_path in &self.extra_include_dirs {
            let _ = cmd.arg(concat_string!("-I", inc_path.to_string_lossy()));
        }
        for def in &self.extra_cpp_defs {
            let s = if let Some(ref v) = def.1 {
                concat_string!("-D", def.0, "=", v)
            } else {
                concat_string!("-D", def.0)
            };
            let _ = cmd.arg(s);
        }
        for undef in &self.cpp_undefs {
            let _ = cmd.arg(concat_string!("-U", undef));
        }
        // Common options.
        let rc_filename = rc_file
            .as_ref()
            .file_name()
            .expect("invalid input filename");
        let mut libname = OsString::from("lib");
        libname.push(&rc_filename);
        let out_file = Path::new(&env::var_os("OUT_DIR")
            .expect("`OUT_DIR` is invalid or not set"))
            .join(libname)
            .with_extension("res.a");
        let _ = cmd.args(&[
            OsStr::new("-Ocoff"),
            OsStr::new("-v"),
            OsStr::new("-c65001"),
            rc_file.as_ref().as_ref(),
            out_file.as_ref(),
        ]);
        cmd.status().and_then(|status| {
            if status.success() {
                let mut res_filename = Path::new(rc_filename).to_owned();
                let _ = res_filename.set_extension("res");
                let stdout = io::stdout();
                let mut stdout_lock = stdout.lock();
                return stdout_lock.write_all(
                    concat_string!(
                        "cargo:rustc-link-search=native=",
                        out_file.parent().expect("empty parent").to_string_lossy(),
                        "\n",
                        "cargo:rustc-link-lib=static=",
                        res_filename.to_string_lossy(),
                        "\n",
                        "cargo:rerun-if-changed=",
                        rc_file.as_ref().to_string_lossy(),
                        "\n"
                    ).as_bytes(),
                );
            }
            let e = if let Some(code) = status.code() {
                io::Error::new(
                    io::ErrorKind::Other,
                    concat_string!("windres.exe returned exit code ", code.to_string()),
                )
            } else {
                io::Error::new(
                    io::ErrorKind::Interrupted,
                    "child process terminated by signal",
                )
            };
            Err(e)
        })
    }
}
