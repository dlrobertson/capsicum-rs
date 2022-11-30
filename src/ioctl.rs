// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{convert::TryFrom, os::unix::io::AsRawFd};

use libc::u_long;

use crate::common::{CapErr, CapErrType, CapResult, CapRights};

const CAP_IOCTLS_ALL: isize = isize::max_value();

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

#[derive(Debug, Default, Eq, PartialEq)]
pub struct IoctlRights(Vec<u_long>);

impl IoctlRights {
    pub fn new(rights: Vec<u_long>) -> IoctlRights {
        IoctlRights(rights)
    }

    pub fn from_file<T: AsRawFd>(fd: &T, len: usize) -> CapResult<IoctlRights> {
        unsafe {
            let mut cmds = Vec::with_capacity(len);
            let res = cap_ioctls_get(fd.as_raw_fd(), cmds.as_mut_ptr(), len);
            if res == CAP_IOCTLS_ALL {
                todo!()
            } else if let Ok(rlen) = usize::try_from(res) {
                if rlen > len {
                    panic!("cap_ioctls_get overflowed our buffer")
                } else {
                    cmds.set_len(rlen);
                    Ok(IoctlRights(cmds))
                }
            } else {
                Err(CapErr::from(CapErrType::Get))
            }
        }
    }
}

impl CapRights for IoctlRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()> {
        unsafe {
            let len = self.0.len();
            if cap_ioctls_limit(fd.as_raw_fd(), self.0.as_ptr(), len) < 0 {
                Err(CapErr::from(CapErrType::Limit))
            } else {
                Ok(())
            }
        }
    }
}

extern "C" {
    fn cap_ioctls_limit(fd: i32, cmds: *const libc::u_long, ncmds: usize) -> i32;
    fn cap_ioctls_get(fd: i32, cmds: *mut libc::u_long, maxcmds: usize) -> isize;
}
