// Copyright (c) 2017-2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! MSVC implementation details.

use super::Build;
use find_winsdk::{SdkInfo, SdkVersion};
use std::env;
use std::ffi::{OsStr, OsString};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_arch = "x86")]
const RC_EXE: &str = "x86/rc.exe";
#[cfg(target_arch = "x86_64")]
const RC_EXE: &str = "x64/rc.exe";
#[cfg(target_arch = "arm")]
const RC_EXE: &str = "arm/rc.exe";
#[cfg(target_arch = "aarch64")]
const RC_EXE: &str = "arm64/rc.exe";

impl Build {
    /// Locates the tool used to compile resources.
    pub(crate) fn find_resource_compiler() -> io::Result<PathBuf> {
        if let Some(bin_path) = env::var_os("WindowsSdkVerBinPath") {
            Ok(Path::new(&bin_path).join(RC_EXE))
        } else {
            match SdkInfo::find(SdkVersion::Any) {
                Ok(Some(info)) => {
                    let path_suffix = if info.product_version().starts_with("10.") {
                        concat_string!("bin/", info.product_version(), ".0/", RC_EXE)
                    } else {
                        concat_string!("bin/", RC_EXE)
                    };
                    Ok(Path::new(info.installation_folder()).join(path_suffix))
                }
                Ok(None) => Err(io::Error::new(
                    ErrorKind::NotFound,
                    "could not locate a Windows SDK installation",
                )),
                Err(e) => Err(e),
            }
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
            let _ = cmd.arg(concat_string!("/i", inc_path.to_string_lossy()));
        }
        for def in &self.extra_cpp_defs {
            let s = if let Some(ref v) = def.1 {
                concat_string!("/d", def.0, "=", v)
            } else {
                concat_string!("/d", def.0)
            };
            let _ = cmd.arg(s);
        }
        for undef in &self.cpp_undefs {
            let _ = cmd.arg(concat_string!("/u", undef));
        }
        // Common options.
        let rc_filename = rc_file
            .as_ref()
            .file_name()
            .expect("invalid input filename");
        let out_file = Path::new(&env::var_os("OUT_DIR").expect("`OUT_DIR` is invalid or not set"))
            .join(rc_filename)
            .with_extension("res.lib");
        let mut fo = OsString::from("/fo");
        fo.push(&out_file);
        let _ = cmd.args(&[
            &fo,
            OsStr::new("/v"),
            OsStr::new("/nologo"),
            OsStr::new("/c65001"),
            rc_file.as_ref().as_ref(),
        ]);
        cmd.status().and_then(|status| {
            if status.success() {
                let stdout = io::stdout();
                let mut stdout_lock = stdout.lock();
                return stdout_lock.write_all(
                    concat_string!(
                        "cargo:rustc-link-search=native=",
                        out_file.parent().expect("empty parent").to_string_lossy(),
                        "\n",
                        "cargo:rustc-link-lib=",
                        out_file
                            .file_stem()
                            .expect("empty filename")
                            .to_string_lossy(),
                        "\n",
                        "cargo:rerun-if-changed=",
                        rc_file.as_ref().to_string_lossy(),
                        "\n"
                    )
                    .as_bytes(),
                );
            }
            let e = if let Some(code) = status.code() {
                io::Error::new(
                    io::ErrorKind::Other,
                    concat_string!("rc.exe returned exit code ", code.to_string()),
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
