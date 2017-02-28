// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use libc::{mode_t, openat, c_uint, c_int};
use std::io;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};

use common::{CapResult, CapErr, CapErrType};

/// Directory with a set of capabilities.
///
/// # Examples
///
/// ```
/// use std::ffi::CString;
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
/// let path = CString::new("lib.rs").unwrap();
/// let fd = dir.open_file(path, 0, None).unwrap();
/// ```
pub struct Directory {
    file: File,
}

impl Directory {
    pub fn new(path: &str) -> io::Result<Directory> {
        let file = try!(File::open(path));
        Ok(Directory {
            file: file,
        })
    }

    pub fn open_file(&self, path: CString, flags: c_int, mode: Option<mode_t>) -> CapResult<File> {
        unsafe {
            let fd = match mode {
                Some(mode) => openat(self.file.as_raw_fd(), path.as_ptr(), flags, mode as c_uint),
                None => openat(self.file.as_raw_fd(), path.as_ptr(), 0)
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
