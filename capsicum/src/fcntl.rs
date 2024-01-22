// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{io, os::unix::io::AsRawFd};

use crate::common::CapRights;

// TODO: use values from libc
/// Fcntl commands that may be limited on file descriptors.
///
/// Note that [`fcntl(2)`](https://www.freebsd.org/cgi/man.cgi?query=fcntl)
/// supports additional commands not listed here.  Those commands are always
/// available and cannot be limited.
#[repr(u32)]
#[derive(Debug)]
pub enum Fcntl {
    /// Get descriptor status flags.
    GetFL = 0x8,
    /// Set descriptor status flags.
    SetFL = 0x10,
    /// Get the process ID or process group currently receiving SIGIO and SIGURG
    /// signals.
    GetOwn = 0x20,
    /// Set the process or process group to receive SIGIO and SIGURG signal.
    SetOwn = 0x40,
}

/// Used to construct a new set of allowed fcntl commands.
///
/// # Example
/// ```
/// # use capsicum::{Fcntl, FcntlsBuilder};
/// let rights = FcntlsBuilder::new()
///     .add(Fcntl::GetFL)
///     .add(Fcntl::SetFL)
///     .finalize();
/// ```
#[derive(Clone, Debug, Default)]
pub struct FcntlsBuilder(u32);

impl FcntlsBuilder {
    pub fn new() -> FcntlsBuilder {
        FcntlsBuilder::default()
    }

    /// Allow an additional fcntl
    ///
    /// # Examples
    /// ```
    /// # use capsicum::{Fcntl, FcntlsBuilder};
    ///
    /// let mut builder = FcntlsBuilder::new();
    /// builder.add(Fcntl::GetFL);
    /// ```
    pub fn add(&mut self, right: Fcntl) -> &mut FcntlsBuilder {
        self.0 |= right as u32;
        self
    }

    pub fn finalize(&self) -> FcntlRights {
        FcntlRights(self.0)
    }

    #[deprecated(since = "0.4.0", note = "If you still need this method, please file an issue at https://github.com/dlrobertson/capsicum-rs/issues")]
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Remove an allowed fcntl from the builder's list.
    ///
    /// # Example
    /// ```
    /// # use capsicum::{Fcntl, FcntlsBuilder};
    /// let mut common_builder = FcntlsBuilder::new();
    /// common_builder.add(Fcntl::GetFL);
    /// common_builder.add(Fcntl::SetFL);
    /// let mut restricted_builder = common_builder.clone();
    /// restricted_builder.remove(Fcntl::SetFL);
    /// ```
    pub fn remove(&mut self, right: Fcntl) -> &mut FcntlsBuilder {
        self.0 &= !(right as u32);
        self
    }
}

/// Used to limit which
/// [`fcntl(2)`](https://www.freebsd.org/cgi/man.cgi?query=fcntl) commands can be
/// used on a file in capability mode.
///
/// # See Also
/// [`cap_fcntls_limit(2)`](https://www.freebsd.org/cgi/man.cgi?query=cap_fcntls_limit)
///
/// # Example
/// ```
/// # use std::os::unix::io::AsRawFd;
/// # use capsicum::{CapRights, FcntlsBuilder, Fcntl};
/// # use tempfile::tempfile;
/// use nix::errno::Errno;
/// use nix::fcntl::{FcntlArg, OFlag, fcntl};
/// let file = tempfile().unwrap();
/// let rights = FcntlsBuilder::new()
///     .add(Fcntl::GetFL)
///     .finalize();
///
/// rights.limit(&file).unwrap();
///
/// capsicum::enter().unwrap();
///
/// fcntl(file.as_raw_fd(), FcntlArg::F_GETFL).unwrap();
///
/// let r = fcntl(file.as_raw_fd(), FcntlArg::F_SETFL(OFlag::O_CLOEXEC));
/// assert_eq!(r, Err(Errno::ENOTCAPABLE));
/// ```
#[derive(Debug, Default, Eq, PartialEq)]
pub struct FcntlRights(u32);

impl FcntlRights {
    #[allow(missing_docs)]
    #[deprecated(since = "0.4.0", note = "use FcntlsBuilder insted")]
    pub fn new(right: u32) -> FcntlRights {
        FcntlRights(right)
    }

    pub fn from_file<T: AsRawFd>(fd: &T) -> io::Result<FcntlRights> {
        unsafe {
            let mut empty_fcntls = 0;
            let res = libc::cap_fcntls_get(fd.as_raw_fd(), &mut empty_fcntls as *mut u32);
            if res < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(FcntlRights(empty_fcntls))
            }
        }
    }
}

impl CapRights for FcntlRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> io::Result<()> {
        unsafe {
            if libc::cap_fcntls_limit(fd.as_raw_fd(), self.0) < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}
