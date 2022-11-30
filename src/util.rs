// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    ffi::CString,
    fs::File,
    io,
    os::unix::{
        ffi::OsStrExt,
        io::{AsRawFd, FromRawFd, RawFd},
    },
    path::Path,
};

use libc::{c_int, c_uint, mode_t, openat};

use crate::common::{CapErr, CapErrType, CapResult};

/// Directory with a set of capabilities.
///
/// # Examples
///
/// ```
/// use capsicum::{self, CapRights, Right, RightsBuilder};
/// use capsicum::util::Directory;
///
/// // Open the directory
/// let dir = Directory::new("./src").unwrap();
///
/// // Create the set of capabilities
/// let rights = RightsBuilder::new(Right::Read)
///     .add(Right::Lookup)
///     .finalize().unwrap();
///
/// // Limit the capabilities
/// rights.limit(&dir).unwrap();
///
/// // Enter the sandbox
/// capsicum::enter().unwrap();
///
/// // Since we have "Lookup" capabilities we can open a file
/// // within the ./src directory
/// let fd = dir.open_file("lib.rs", 0, None).unwrap();
/// ```
pub struct Directory {
    file: File,
}

impl Directory {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Directory> {
        let file = File::open(path.as_ref())?;
        Ok(Directory { file })
    }

    pub fn open_file<P: AsRef<Path> + ?Sized>(
        &self,
        path: &P,
        flags: c_int,
        mode: Option<mode_t>,
    ) -> CapResult<File> {
        let p = CString::new(path.as_ref().as_os_str().as_bytes()).map_err(CapErr::Nul)?;
        unsafe {
            let fd = match mode {
                Some(mode) => openat(self.file.as_raw_fd(), p.as_ptr(), flags, mode as c_uint),
                None => openat(self.file.as_raw_fd(), p.as_ptr(), 0),
            };
            if fd < 0 {
                Err(CapErr::from(CapErrType::Invalid))
            } else {
                Ok(File::from_raw_fd(fd))
            }
        }
    }
}

impl FromRawFd for Directory {
    unsafe fn from_raw_fd(fd: RawFd) -> Directory {
        Directory {
            file: File::from_raw_fd(fd),
        }
    }
}

impl AsRawFd for Directory {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}
