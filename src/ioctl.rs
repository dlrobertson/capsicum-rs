// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{convert::TryFrom, os::unix::io::AsRawFd};

use libc::u_long;

use crate::common::{CapErr, CapErrType, CapResult, CapRights};

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

/// A set of commands  commands that can be allowed with
/// [`ioctl`](https://www.freebsd.org/cgi/man.cgi?query=ioctl) in capability
/// mode.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum IoctlRights {
    #[default]
    Unlimited,
    Limited(Vec<u_long>),
}

impl IoctlRights {
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
    pub fn from_file<T: AsRawFd>(fd: &T, len: usize) -> CapResult<IoctlRights> {
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
                Err(CapErr::from(CapErrType::Get))
            }
        }
    }
}

impl CapRights for IoctlRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()> {
        if let IoctlRights::Limited(v) = self {
            let len = v.len();
            unsafe {
                if libc::cap_ioctls_limit(fd.as_raw_fd(), v.as_ptr(), len) < 0 {
                    return Err(CapErr::from(CapErrType::Limit));
                }
            }
        }
        Ok(())
    }
}
