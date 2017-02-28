// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use common::{CapErr, CapErrType, CapResult, CapRights};
use std::os::unix::io::AsRawFd;

#[repr(u32)]
#[derive(Debug)]
pub enum Fcntl {
    GetFL = 0x8,
    SetFL = 0x10,
    GetOwn = 0x20,
    SetOwn = 0x40,
}

#[derive(Debug, Default)]
pub struct FcntlsBuilder(u32);

impl FcntlsBuilder {
    pub fn new(right: Fcntl) -> FcntlsBuilder {
        FcntlsBuilder(right as u32)
    }

    pub fn add<'a>(&'a mut self, right: Fcntl) -> &'a mut FcntlsBuilder {
        self.0 |= right as u32;
        self
    }

    pub fn finalize(&self) -> FcntlRights {
        FcntlRights::new(self.0)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }

    pub fn remove<'a>(&'a mut self, right: Fcntl) -> &'a mut FcntlsBuilder {
        self.0 = self.0 & (!(right as u32));
        self
    }
}

#[derive(Debug, Default)]
pub struct FcntlRights(u32);

impl FcntlRights {
    pub fn new(right: u32) -> FcntlRights {
        FcntlRights(right)
    }

    pub fn from_file<T: AsRawFd>(fd: &T) -> CapResult<FcntlRights> {
        unsafe {
            let mut empty_fcntls = 0;
            let res = cap_fcntls_get(fd.as_raw_fd() as isize, &mut empty_fcntls as *mut u32);
            if res < 0 {
                Err(CapErr::from(CapErrType::Get))
            } else {
                Ok(FcntlRights(empty_fcntls))
            }
        }
    }
}

impl CapRights for FcntlRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()> {
        unsafe {
            if cap_fcntls_limit(fd.as_raw_fd() as isize, self.0 as u32) < 0 {
                Err(CapErr::from(CapErrType::Get))
            } else {
                Ok(())
            }
        }
    }
}

impl PartialEq for FcntlRights {
    fn eq(&self, other: &FcntlRights) -> bool {
        self.0 == other.0
    }
}

extern "C" {
    fn cap_fcntls_limit(fd: isize, fcntlrights: u32) -> isize;
    fn cap_fcntls_get(fd: isize, fcntlrightsp: *mut u32) -> isize;
}
