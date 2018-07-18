# windres-rs

[![Travis CI](https://travis-ci.org/FaultyRAM/windres-rs.svg)][1]
[![AppVeyor](https://ci.appveyor.com/api/projects/status/ffj632w0xqh98c83?retina=true&svg=true)][2]
[![Crates.io](https://img.shields.io/crates/v/windres.svg)][3]
[![Docs.rs](https://docs.rs/windres/badge.svg)][4]

`windres` is a [Rust][5] library crate for compiling [Windows resource (.rc) files][6] into object
files at build time, which are then forwarded to the linker. This allows for embedding icons,
version information, native UI data, etc. in binaries compiled from Rust code.

## Example

The following example demonstrates how to embed an icon in a binary crate:

```rc
// hello-world.rc

1 ICON "hello-world.ico"
```

```rust
// build.rs

extern crate windres;

use windres::Build;

fn main() {
    Build::new().compile("hello-world.rc").unwrap();
}
```

## Usage

You need to install the resource compiler for your target ABI in order for `windres` to work.
Currently the following compilers are supported:

* `windres.exe` for GNU targets (included in the [mingw-w64][7] toolchain)
* `rc.exe` for MSVC targets (included in the Windows SDK, which can be installed via Visual Studio
  Installer)

Once the appropriate resource compiler is installed, add `windres` as a build dependency in
`Cargo.toml`:

```toml
[target.'cfg(windows)'.build-dependencies]
windres = "0.2"
```

Then, create a build script (if you haven't already) and add a reference to `windres`:

```rust
#[cfg(windows)]
extern crate windres;
```

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[1]: https://travis-ci.org/FaultyRAM/windres-rs
[2]: https://ci.appveyor.com/project/FaultyRAM/windres-rs
[3]: https://crates.io/crates/windres
[4]: https://docs.rs/windres
[5]: https://www.rust-lang.org
[6]: https://msdn.microsoft.com/en-us/library/windows/desktop/aa380599(v=vs.85).aspx
[7]: https://mingw-w64.org
[8]: https://www.visualstudio.com
