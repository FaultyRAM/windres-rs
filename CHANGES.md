# windres-rs Changelog

## 0.2.2 (27/02/2021)

* Previous versions of this crate were broken for MSVC targets by a breaking change in Rust (see
  https://github.com/rust-lang/rust/pull/72785). This version adds a workaround for the breakage.
* To prevent future breakage, trivial changes were made to parts of the code. This is a refactor,
  i.e. no change in behaviour from the previous version, but the minimum working Rust version may
  have increased as a result.

## 0.2.1 (18/08/2018)

* Update find-winsdk to 0.2. This fixes Windows 8.1A SDK support and adds support for earlier
  versions of the Windows SDK through to v6.0.

## 0.2.0 (1/08/2018)

* The strategy for detecting rc.exe on MSVC ABI targets has changed. Now windres tries the
  following, in order:

  1. The `WindowsSdkVerBinPath` environment variable
  2. The `WindowsSdkDir` environment variable
  3. The Windows 10 SDK bin path for the target arch, via registry query
  4. The Windows 8.1 SDK bin path for the target arch, via registry query

* The optional dependency on clippy has been removed. Use `cargo clippy` instead.

## 0.1.0 (20/10/2017)

* Initial release.
