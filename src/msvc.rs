// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! MSVC implementation details.

use std::{env, ptr};
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::process::Command;
use Build;

/// Generate a wide string.
macro_rules! wide_string {
    ($s:expr) => {{
        use std::iter;
        let v: Vec<u16> = OsStr::new($s).encode_wide().chain(iter::once(0)).collect();
        v
    }}
}

#[cfg(target_arch = "x86")]
/// Relative path to rc.exe for the target ABI.
const RC_EXE: &str = "x86/rc.exe";
#[cfg(target_arch = "x86_64")]
/// Relative path to rc.exe for the target ABI.
const RC_EXE: &str = "x64/rc.exe";
#[cfg(target_arch = "arm")]
/// Relative path to rc.exe for the target ABI.
const RC_EXE: &str = "arm/rc.exe";
#[cfg(target_arch = "aarch64")]
/// Relative path to rc.exe for the target ABI.
const RC_EXE: &str = "arm64/rc.exe";

/// Registry subkey for Windows 10 SDK installation data.
const SUBKEY_WINSDK_10_0: &str = r"SOFTWARE\Microsoft\Microsoft SDKs\Windows\v10.0";

/// Registry subkey for Windows 8.1 SDK installation data.
const SUBKEY_WINSDK_8_1: &str = r"SOFTWARE\Microsoft\Microsoft SDKs\Windows\v8.1";

/// An unsigned 32-bit integer.
type DWORD = u32;
/// A registry key handle.
type HKEY = PVOID;
/// A signed 32-bit integer.
type LONG = i32;
/// A constant pointer to a wide string.
type LPCWSTR = *const u16;
/// A mutable pointer to an unsigned 32-bit integer.
type LPDWORD = *mut DWORD;
/// A mutable pointer to arbitrary data.
type PVOID = *mut u8;

/// The registry key for the current user.
const HKEY_CURRENT_USER: HKEY = 0x8000_0001 as HKEY;
/// The registry key for the current machine.
const HKEY_LOCAL_MACHINE: HKEY = 0x8000_0002 as HKEY;
/// A flag instructing `RegGetValueEx` to fail if the type of a registry value is not `REG_SZ`.
const RRF_RT_REG_SZ: DWORD = 0x0000_0002;
/// A flag instructing `RegGetValueEx` to search the `WOW6432Node` subkey on 64-bit Windows.
const RRF_SUBKEY_WOW6432KEY: DWORD = 0x0002_0000;

#[link(name = "advapi32")]
extern "system" {
    fn RegGetValueW(
        key: HKEY,
        subkey: LPCWSTR,
        value: LPCWSTR,
        flags: DWORD,
        value_type: LPDWORD,
        value: PVOID,
        len: LPDWORD,
    ) -> LONG;
}

impl Build {
    /// Locates the tool used to compile resources.
    pub(crate) fn find_resource_compiler() -> io::Result<PathBuf> {
        // If `VCINSTALLDIR` is set, the user has likely invoked either vcvars or VsDevCmd, in
        // which case rc.exe should already be present in `PATH`.
        if env::var_os("VCINSTALLDIR").is_some() {
            if let Some(p) = env::var_os("PATH").and_then(|path| {
                env::split_paths(&path)
                    .map(|p| p.join("rc.exe"))
                    .find(|p| p.exists())
            }) {
                Ok(p)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "`VCINSTALLDIR` was set but `rc.exe` was not found",
                ))
            }
        } else {
            Self::get_rc_path_10_0().or_else(|_| Self::get_rc_path_8_1())
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
        let out_file = Path::new(&env::var_os("OUT_DIR")
            .expect("`OUT_DIR` is invalid or not set"))
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
                        "cargo:rustc-link-lib=static=",
                        out_file
                            .file_stem()
                            .expect("empty filename")
                            .to_string_lossy(),
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

    /// Constructs a path to rc.exe in the latest Windows 10 SDK.
    fn get_rc_path_10_0() -> io::Result<PathBuf> {
        let subkey = wide_string!(SUBKEY_WINSDK_10_0);
        let folder_value = wide_string!("InstallationFolder");
        let version_value = wide_string!("ProductVersion");
        let sdk_root = Self::reg_find_string(&subkey, &folder_value)?;
        let sdk_version = Self::reg_find_string(&subkey, &version_value).map(|mut s| {
            // Windows SDK versions don't seem to use the build number, so just stick a zero on
            // the end.
            // (Now watch as Microsoft publish an update that increments the build number...)
            s.push(".0");
            s
        })?;
        Ok(
            Path::new(&sdk_root)
                .join("bin")
                .join(sdk_version)
                .join(RC_EXE),
        )
    }

    /// Constructs a path to rc.exe in the Windows 8.1 SDK.
    fn get_rc_path_8_1() -> io::Result<PathBuf> {
        let subkey = wide_string!(SUBKEY_WINSDK_8_1);
        let folder_value = wide_string!("InstallationFolder");
        let sdk_root = Self::reg_find_string(&subkey, &folder_value)?;
        Ok(Path::new(&sdk_root).join("bin").join(RC_EXE))
    }

    /// Looks for a string value in HKLM, or in HKCU if not found.
    fn reg_find_string(subkey: &[u16], value: &[u16]) -> io::Result<OsString> {
        Self::reg_get_string(HKEY_LOCAL_MACHINE, subkey, value)
            .or_else(|_| Self::reg_get_string(HKEY_CURRENT_USER, subkey, value))
            .map_err(io::Error::from_raw_os_error)
    }

    #[cfg_attr(feature = "clippy", allow(cast_possible_truncation))]
    #[cfg_attr(feature = "clippy", allow(indexing_slicing))]
    #[cfg_attr(feature = "clippy", allow(integer_arithmetic))]
    /// Safe `RegGetValueW` wrapper for obtaining strings.
    fn reg_get_string(key: HKEY, subkey: &[u16], value: &[u16]) -> Result<OsString, LONG> {
        unsafe {
            let mut buf: [u16; 32_768] = [0; 32_768];
            let mut buf_len = buf.len() as DWORD;
            let result = RegGetValueW(
                key,
                subkey.as_ptr(),
                value.as_ptr(),
                RRF_RT_REG_SZ | RRF_SUBKEY_WOW6432KEY,
                ptr::null_mut(),
                buf.as_mut_ptr() as PVOID,
                &mut buf_len,
            );
            if result == 0 {
                Ok(OsString::from_wide(&buf[..buf_len as usize / 2 - 1]))
            } else {
                Err(result)
            }
        }
    }
}
