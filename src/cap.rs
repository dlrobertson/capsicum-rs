// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]
#![allow(non_camel_case_types)]


use right::{RIGHTS_VERSION, Right};
use std::fs::File;
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct Rights(cap_rights_t);

impl Rights {
    pub fn new(raw_rights: u64) -> Result<Rights, ()> {
        unsafe {
            let mut empty_rights = cap_rights_t { cr_rights: [0; RIGHTS_VERSION + 2] };
            let rights_ptr = __cap_rights_init(RIGHTS_VERSION,
                                               &mut empty_rights as *mut cap_rights_t,
                                               raw_rights,
                                               0u64);
            if rights_ptr.is_null() {
                Err(())
            } else {
                let rights = Rights(empty_rights);
                if rights.is_valid() {
                    Ok(rights)
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn from_file(file: &File) -> Result<Rights, ()> {
        unsafe {
            let mut empty_rights = cap_rights_t { cr_rights: [0; RIGHTS_VERSION + 2] };
            let res = __cap_rights_get(RIGHTS_VERSION,
                                       file.as_raw_fd() as isize,
                                       &mut empty_rights as *mut cap_rights_t);
            if res < 0 {
                Err(())
            } else {
                let rights = Rights(empty_rights);
                if rights.is_valid() {
                    Ok(rights)
                } else {
                    Err(())
                }
            }
        }
    }

    pub fn contains(&self, other: &Rights) -> bool {
        unsafe { cap_rights_contains(&self.0, &other.0) }
    }

    pub fn is_set(&self, raw_rights: Right) -> bool {
        unsafe { __cap_rights_is_set(&self.0 as *const cap_rights_t, raw_rights as u64, 0u64) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { cap_rights_is_valid(&self.0) }
    }

    pub fn limit(&self, file: &File) -> isize {
        unsafe {
            let fd = file.as_raw_fd() as isize;;
            cap_rights_limit(fd, &self.0 as *const cap_rights_t)
        }
    }

    pub fn merge(&mut self, other: &Rights) -> Result<(), ()> {
        unsafe {
            let result = cap_rights_merge(&mut self.0 as *mut cap_rights_t, &other.0);
            if result.is_null() {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, other: &Rights) -> Result<(), ()> {
        unsafe {
            let result = cap_rights_remove(&mut self.0 as *mut cap_rights_t, &other.0);
            if result.is_null() {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn set(&mut self, raw_rights: Right) -> Result<(), ()> {
        unsafe {
            let result =
                __cap_rights_set(&mut self.0 as *mut cap_rights_t, raw_rights as u64, 0u64);
            if result.is_null() {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn clear(&mut self, raw_rights: Right) -> Result<(), ()> {
        unsafe {
            let result =
                __cap_rights_clear(&mut self.0 as *mut cap_rights_t, raw_rights as u64, 0u64);
            if result.is_null() {
                Err(())
            } else {
                Ok(())
            }
        }
    }
}

impl PartialEq for Rights {
    fn eq(&self, other: &Rights) -> bool {
        self.0.cr_rights == other.0.cr_rights
    }
}

pub struct RightsBuilder(u64);

impl RightsBuilder {
    pub fn new(right: Right) -> RightsBuilder {
        RightsBuilder(right as u64)
    }

    pub fn empty() -> RightsBuilder {
        RightsBuilder(0)
    }

    pub fn add<'a>(&'a mut self, right: Right) -> &'a mut RightsBuilder {
        self.0 |= right as u64;
        self
    }

    pub fn remove<'a>(&'a mut self, right: Right) -> &'a mut RightsBuilder {
        self.0 = (self.0 & !(right as u64)) | 0x200000000000000;
        self
    }

    pub fn finalize(self) -> Result<Rights, ()> {
        Rights::new(self.0)
    }

    pub fn bits(&self) -> u64 {
        self.0
    }
}

pub fn enter() -> Result<(), ()> {
    if unsafe { cap_enter() } < 0 {
        Err(())
    } else {
        Ok(())
    }
}

pub fn sandboxed() -> bool {
    if unsafe { cap_sandboxed() } == 1 {
        true
    } else {
        false
    }
}

pub fn get_mode() -> Result<usize, ()> {
    let mut mode = 0;
    unsafe {
        if cap_getmode(&mut mode as *mut usize) != 0 {
            return Err(());
        }
    }
    Ok(mode)
}

#[repr(C)]
#[derive(Debug)]
pub struct cap_rights_t {
    cr_rights: [u64; RIGHTS_VERSION + 2],
}

type cap_rights = cap_rights_t;

extern "C" {
    fn cap_rights_is_valid(rights: *const cap_rights_t) -> bool;
    fn cap_rights_merge(dst: *mut cap_rights_t, src: *const cap_rights_t) -> *mut cap_rights_t;
    fn cap_rights_remove(dst: *mut cap_rights_t, src: *const cap_rights_t) -> *mut cap_rights_t;
    fn cap_rights_contains(big: *const cap_rights_t, little: *const cap_rights_t) -> bool;
    fn cap_rights_limit(fd: isize, rights: *const cap_rights_t) -> isize;
    fn __cap_rights_init(version: usize,
                         rights: *mut cap_rights_t,
                         raw_rights: u64,
                         sentinel: u64)
                         -> *mut cap_rights_t;
    fn __cap_rights_set(rights: *mut cap_rights_t,
                        raw_rights: u64,
                        sentinel: u64)
                        -> *mut cap_rights_t;
    fn __cap_rights_clear(rights: *mut cap_rights_t,
                          raw_rights: u64,
                          sentinel: u64)
                          -> *mut cap_rights_t;
    fn __cap_rights_is_set(rights: *const cap_rights_t, raw_rights: u64, sentinel: u64) -> bool;
    fn cap_enter() -> isize;
    fn cap_sandboxed() -> isize;
    fn cap_getmode(modep: *mut usize) -> isize;
    fn __cap_rights_get(version: usize, fd: isize, rightsp: *mut cap_rights_t) -> isize;
}
