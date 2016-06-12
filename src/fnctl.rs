// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::os::unix::io::AsRawFd;

#[repr(u32)]
pub enum Fcntl {
    GetFL = 0x8,
    SetFL = 0x10,
    GetOwn = 0x20,
    SetOwn = 0x40
}

pub struct FcntlsBuilder(u32);

impl FcntlsBuilder {
    pub fn new(right: Fcntl) -> FcntlsBuilder {
        FcntlsBuilder(right as u32)
    }

    pub fn empty() -> FcntlsBuilder {
        FcntlsBuilder(0)
    }

    pub fn add<'a>(&'a mut self, right: Fcntl) -> &'a mut FcntlsBuilder {
        self.0 |= right as u32;
        self
    }

    pub fn remove<'a>(&'a mut self, right: Fcntl) -> &'a mut FcntlsBuilder {
        self.0 = self.0 & (!(right as u32));
        self
    }

    pub fn finalize(&self) -> Fcntls {
        Fcntls::new(self.0)
    }

    pub fn bits(&self) -> u32 {
        self.0
    }
}

pub struct Fcntls(u32);

impl Fcntls {
    pub fn new(right: u32) -> Fcntls {
        Fcntls(right)
    }

    pub fn from_file(file: &File) -> Result<Fcntls, ()> {
        unsafe {
            let mut empty_fcntls = 0;
            let res = cap_fcntls_get(file.as_raw_fd() as isize,
                                     &mut empty_fcntls as *mut u32);
            if res < 0 {
                Err(())
            } else {
                Ok(Fcntls(empty_fcntls))
            }
        }
    }

    pub fn limit(&self, file: &File) -> isize {
        unsafe {
            let fd = file.as_raw_fd() as isize;;
            cap_fcntls_limit(fd, self.0 as u32)
        }
    }
}

extern "C" {
    fn cap_fcntls_limit(fd: isize, fcntlrights: u32) -> isize;
    fn cap_fcntls_get(fd: isize, fcntlrightsp: *mut u32) -> isize;
}
