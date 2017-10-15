// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! GNU implementation details.

use std::io;
use std::path::PathBuf;
use std::process::ExitStatus;
use Build;

impl Build {
    /// Locates the tool used to compile resources.
    pub(crate) fn find_resource_compiler() -> io::Result<PathBuf> {
        unimplemented!()
    }

    /// Invokes the resource compiler using the current arguments.
    pub(crate) fn compile_resource<P: AsRef<Path>>(
        &self,
        _rc_file: P,
        _compiler: PathBuf,
    ) -> io::Result<()> {
        unimplemented!()
    }
}
