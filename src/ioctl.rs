// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::os::unix::io::AsRawFd;
use common::{CapErr, CapErrType, CapResult, CapRights};

#[derive(Debug, Default)]
pub struct IoctlsBuilder(Vec<u64>);

impl IoctlsBuilder {
    pub fn new(right: u64) -> IoctlsBuilder {
        IoctlsBuilder(vec![right])
    }

    pub fn add(&mut self, right: u64) -> &mut IoctlsBuilder {
        self.0.push(right);
        self
    }

    pub fn raw(&self) -> Vec<u64> {
        self.0.clone()
    }

    pub fn remove(&mut self, right: u64) -> &mut IoctlsBuilder {
        self.0.retain(|&item| item != right);
        self
    }

    pub fn finalize(&self) -> IoctlRights {
        IoctlRights::new(self.0.clone())
    }
}

#[derive(Debug, Default)]
pub struct IoctlRights(Vec<u64>);

impl IoctlRights {
    pub fn new(rights: Vec<u64>) -> IoctlRights {
        IoctlRights(rights)
    }

    pub fn from_file<T: AsRawFd>(fd: &T, len: usize) -> CapResult<IoctlRights> {
        unsafe {
            let empty_ioctls = Vec::with_capacity(len).as_mut_ptr();
            let res = cap_ioctls_get(fd.as_raw_fd() as isize, empty_ioctls, len);
            let res_vec = Vec::from_raw_parts(empty_ioctls, len, len);
            if res < 0 {
                Err(CapErr::from(CapErrType::Get))
            } else {
                Ok(IoctlRights(res_vec))
            }
        }
    }
}

impl CapRights for IoctlRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()> {
        unsafe {
            let len = self.0.len();
            if cap_ioctls_limit(fd.as_raw_fd() as isize, self.0.as_ptr(), len) < 0 {
                Err(CapErr::from(CapErrType::Limit))
            } else {
                Ok(())
            }
        }
    }
}

impl PartialEq for IoctlRights {
    fn eq(&self, other: &IoctlRights) -> bool {
        self.0 == other.0
    }
}

extern "C" {
    fn cap_ioctls_limit(fd: isize, cmds: *const u64, ncmds: usize) -> isize;
    fn cap_ioctls_get(fd: isize, cmds: *mut u64, maxcmds: usize) -> isize;
}
