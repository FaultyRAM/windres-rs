# windres-rs Changelog

## 0.2.0 (WIP)

* The strategy for detecting rc.exe on MSVC ABI targets has changed. Now windres tries the
  following, in order:

  1. The `WindowsSdkVerBinPath` environment variable
  2. The `WindowsSdkDir` environment variable
  3. The Windows 10 SDK bin path for the target arch, via registry query
  4. The Windows 8.1 SDK bin path for the target arch, via registry query

* The optional dependency on clippy has been removed. Use `cargo clippy` instead.

## 0.1.0 (20/10/2017)

* Initial release.
