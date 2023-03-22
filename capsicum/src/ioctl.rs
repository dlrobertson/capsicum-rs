// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{convert::TryFrom, io, os::unix::io::AsRawFd};

use libc::u_long;

use crate::common::CapRights;

const CAP_IOCTLS_ALL: isize = isize::max_value();

/// Used to construct a new set of allowed ioctl commands.
///
/// # Example
/// Using ioctl command codes from libc:
/// ```
/// # use capsicum::IoctlsBuilder;
/// let builder = IoctlsBuilder::new(libc::TIOCGETD);
/// let rights = builder.finalize();
/// ```
/// Declaring ioctl command codes with Nix, for ioctls not present in libc:
/// ```
/// use std::mem;
/// #[macro_use(request_code_read)]
/// extern crate nix;
/// # use capsicum::IoctlsBuilder;
/// const TIOCGETD: libc::u_long = request_code_read!(b't', 26, mem::size_of::<libc::c_int>());
///
/// fn main() {
///     let builder = IoctlsBuilder::new(TIOCGETD);
///     let rights = builder.finalize();
/// }
#[derive(Debug, Default)]
pub struct IoctlsBuilder(Vec<u_long>);

impl IoctlsBuilder {
    pub fn new(right: u_long) -> IoctlsBuilder {
        IoctlsBuilder(vec![right])
    }

    pub fn add(&mut self, right: u_long) -> &mut IoctlsBuilder {
        self.0.push(right);
        self
    }

    // Is this method really necessary?  If it is, I suggest renaming it to
    // "into_raw", and also deriving Clone on the builder.
    pub fn raw(&self) -> Vec<u_long> {
        self.0.clone()
    }

    pub fn remove(&mut self, right: u_long) -> &mut IoctlsBuilder {
        self.0.retain(|&item| item != right);
        self
    }

    pub fn finalize(&self) -> IoctlRights {
        IoctlRights::new(self.0.clone())
    }
}

/// Used to reduce (but never expand) the ioctl commands that may be used on a
/// file descriptor.
///
/// # See Also
/// [`ioctl(2)`](https://www.freebsd.org/cgi/man.cgi?query=ioctl)
/// [`cap_ioctls_limit(2)`](https://www.freebsd.org/cgi/man.cgi?query=cap_ioctls_limit)
///
/// # Example
/// ```
/// # use std::os::unix::io::AsRawFd;
/// # use capsicum::{CapRights, IoctlsBuilder};
/// # use tempfile::tempfile;
/// use std::mem;
/// use libc::{c_int, u_long};
/// use nix::{ioctl_read, request_code_read};
/// use nix::errno::Errno;
/// use nix::sys::socket::{AddressFamily, SockType, SockFlag, socketpair};
///
/// const FIONREAD: u_long = request_code_read!(b'f', 127, mem::size_of::<libc::c_int>());
/// ioctl_read!(fionread, b'f', 127, libc::c_int);
/// ioctl_read!(fionwrite, b'f', 119, libc::c_int);
///
/// let (fd1, fd2) = socketpair(
///     AddressFamily::Unix,
///     SockType::Stream,
///     None,
///     SockFlag::empty()
/// ).unwrap();
/// let rights = IoctlsBuilder::new(FIONREAD)
///     .finalize();
///
/// rights.limit(&fd1).unwrap();
///
/// capsicum::enter().unwrap();
///
/// let mut n: c_int = 0;
/// unsafe{ fionread(fd1.as_raw_fd(), &mut n as *mut c_int) }.unwrap();
///
/// let e = unsafe{ fionwrite(fd1.as_raw_fd(), &mut n as *mut c_int) };
/// assert_eq!(e, Err(Errno::ENOTCAPABLE));
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum IoctlRights {
    /// All ioctl commands will be allowed.
    #[default]
    Unlimited,
    /// Only the included ioctl commands will be allowed.
    Limited(Vec<u_long>),
}

impl IoctlRights {
    // It's pretty hard to use this function correctly.  I suggest removing it
    // from the public API, and forcing people to use IoctlsBuilder instead.
    pub fn new(rights: Vec<u_long>) -> IoctlRights {
        IoctlRights::Limited(rights)
    }

    /// Retrieve the list of currently allowed ioctl commands from a file.
    ///
    /// # Returns
    ///
    /// - `Ok(IoctlRights::Unlimited)`:      All ioctl commands are allowed
    /// - `Ok(IoctlRights::Limited([]))`:    No ioctl commands are allowed
    /// - `Ok(IoctlRights::Limited([...]))`: Only these ioctl commands are allowed.
    /// - `Err(_)`:           Retrieving the list failed.
    pub fn from_file<T: AsRawFd>(fd: &T, len: usize) -> io::Result<IoctlRights> {
        let mut cmds = Vec::with_capacity(len);
        unsafe {
            let res = libc::cap_ioctls_get(fd.as_raw_fd(), cmds.as_mut_ptr(), len);
            if res == CAP_IOCTLS_ALL {
                Ok(IoctlRights::Unlimited)
            } else if let Ok(rlen) = usize::try_from(res) {
                if rlen > len {
                    panic!("cap_ioctls_get overflowed our buffer")
                } else {
                    cmds.set_len(rlen);
                    Ok(IoctlRights::Limited(cmds))
                }
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

impl CapRights for IoctlRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> io::Result<()> {
        if let IoctlRights::Limited(v) = self {
            let len = v.len();
            unsafe {
                if libc::cap_ioctls_limit(fd.as_raw_fd(), v.as_ptr(), len) < 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }
        Ok(())
    }
}
