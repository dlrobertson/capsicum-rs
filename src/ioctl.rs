// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::os::unix::io::AsRawFd;

pub struct IoctlsBuilder(Vec<u64>);

impl IoctlsBuilder {
    pub fn new(right: u64) -> IoctlsBuilder {
        IoctlsBuilder(vec![right])
    }

    pub fn empty() -> IoctlsBuilder {
        IoctlsBuilder(Vec::new())
    }

    pub fn add<'a>(&'a mut self, right: u64) -> &'a mut IoctlsBuilder {
        self.0.push(right);
        self
    }

    pub fn remove<'a>(&'a mut self, right: u64) -> &'a mut IoctlsBuilder {
        self.0.retain(|&item| item != right);
        self
    }

    pub fn finalize(self) -> Ioctls {
        Ioctls::new(self.0)
    }

    pub fn raw(&self) -> Vec<u64> {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct Ioctls(Vec<u64>);

impl Ioctls {
    pub fn new(rights: Vec<u64>) -> Ioctls {
        Ioctls(rights)
    }

    pub fn from_file(file: &File, len: usize) -> Result<Ioctls, ()> {
        unsafe {
            let empty_ioctls = Vec::with_capacity(len).as_mut_ptr();
            let res = cap_ioctls_get(file.as_raw_fd() as isize, empty_ioctls, len);
            let res_vec = Vec::from_raw_parts(empty_ioctls, len, len);
            if res < 0 {
                Err(())
            } else {
                Ok(Ioctls(res_vec))
            }
        }
    }

    pub fn limit(&self, file: &File) -> isize {
        unsafe {
            let fd = file.as_raw_fd() as isize;;
            let len = self.0.len();
            cap_ioctls_limit(fd, self.0.as_ptr(), len)
        }
    }
}

extern "C" {
    fn cap_ioctls_limit(fd: isize, cmds: *const u64, ncmds: usize) -> isize;
    fn cap_ioctls_get(fd: isize, cmds: *mut u64, maxcmds: usize) -> isize;
}
