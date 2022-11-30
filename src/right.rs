// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::{
    io,
    mem,
    ops::BitAnd,
    os::{
        raw::c_char,
        unix::io::{AsRawFd, RawFd},
    },
};

use libc::cap_rights_t;

use crate::common::{CapErr, CapErrType, CapResult, CapRights};

pub const RIGHTS_VERSION: i32 = 0;

macro_rules! cap_right {
    ($idx:expr, $bit:expr) => {
        ((1u64 << (57 + ($idx))) | ($bit))
    };
}

macro_rules! right_or {
    ($($right:expr),*) => {
        $($right as u64)|*
    }
}

#[repr(u64)]
#[derive(Debug)]
pub enum Right {
    Null = 0,
    Read = cap_right!(0, 0x1u64),
    Write = cap_right!(0, 0x2u64),
    SeekTell = cap_right!(0, 0x4u64),
    Seek = right_or!(Right::SeekTell, 0x8u64),
    Pread = right_or!(Right::Seek, Right::Read),
    Pwrite = right_or!(Right::Seek, Right::Write),
    Mmap = cap_right!(0, 0x10u64),
    MmapR = right_or!(Right::Mmap, Right::Seek, Right::Read),
    MmapW = right_or!(Right::Mmap, Right::Seek, Right::Write),
    MmapX = right_or!(Right::Mmap, Right::Seek, 0x20u64),
    MmapRW = right_or!(Right::MmapR, Right::MmapW),
    MmapRX = right_or!(Right::MmapR, Right::MmapX),
    MmapWX = right_or!(Right::MmapW, Right::MmapX),
    MmapRWX = right_or!(Right::MmapR, Right::MmapW, Right::MmapX),
    Create = cap_right!(0, 0x40u64),
    Fexecve = cap_right!(0, 0x80u64),
    Fsync = cap_right!(0, 0x100u64),
    Ftruncate = cap_right!(0, 0x200u64),
    Lookup = cap_right!(0, 0x400u64),
    Fchdir = cap_right!(0, 0x800u64),
    Fchflags = cap_right!(0, 0x1000u64),
    Fchflagsat = right_or!(Right::Fchflags, Right::Lookup),
    Fchmod = cap_right!(0, 0x2000u64),
    Fchmodat = right_or!(Right::Fchmod, Right::Lookup),
    Fchown = cap_right!(0, 0x4000u64),
    Fchownat = right_or!(Right::Fchown, Right::Lookup),
    Fcntl = cap_right!(0, 0x8000u64),
    Flock = cap_right!(0, 0x10000u64),
    Fpathconf = cap_right!(0, 0x20000u64),
    Fsck = cap_right!(0, 0x40000u64),
    Fstat = cap_right!(0, 0x80000u64),
    Fstatat = right_or!(Right::Fstat, Right::Lookup),
    Fstatfs = cap_right!(0, 0x100000u64),
    Futimes = cap_right!(0, 0x200000u64),
    Futimesat = right_or!(Right::Futimes, Right::Lookup),
    Linkat = right_or!(Right::Lookup, 0x400000u64),
    Mkdirat = right_or!(Right::Lookup, 0x800000u64),
    Mkfifoat = right_or!(Right::Lookup, 0x1000000u64),
    Mknotat = right_or!(Right::Lookup, 0x2000000u64),
    Renameat = right_or!(Right::Lookup, 0x4000000u64),
    Symlinkat = right_or!(Right::Lookup, 0x8000000u64),
    Unlinkat = right_or!(Right::Lookup, 0x10000000u64),
    Accept = cap_right!(0, 0x20000000u64),
    Bind = cap_right!(0, 0x40000000u64),
    Connect = cap_right!(0, 0x80000000u64),
    Getpeername = cap_right!(0, 0x100000000u64),
    Getsockname = cap_right!(0, 0x200000000u64),
    Getsockopt = cap_right!(0, 0x400000000u64),
    Listen = cap_right!(0, 0x800000000u64),
    Peeloff = cap_right!(0, 0x1000000000u64),
    Setsockopt = cap_right!(0, 0x2000000000u64),
    Shutdown = cap_right!(0, 0x4000000000u64),
    Bindat = right_or!(Right::Lookup, 0x8000000000u64),
    Connectat = right_or!(Right::Lookup, 0x10000000000u64),
    SockClient = right_or!(
        Right::Connect,
        Right::Getpeername,
        Right::Getsockname,
        Right::Getsockopt,
        Right::Peeloff,
        Right::Read,
        Right::Write,
        Right::Setsockopt,
        Right::Shutdown
    ),
    SockServer = right_or!(
        Right::Accept,
        Right::Bind,
        Right::Getpeername,
        Right::Getsockname,
        Right::Getsockopt,
        Right::Listen,
        Right::Peeloff,
        Right::Read,
        Right::Write,
        Right::Setsockopt,
        Right::Shutdown
    ),
    All0 = cap_right!(0, 0x7FFFFFFFFFu64),
    Unused040 = cap_right!(0, 0u64),
    Unused057 = cap_right!(0, 0x0100000000000000u64),
    MacGet = cap_right!(1, 0x1u64),
    MacSet = cap_right!(1, 0x2u64),
    SemGetvalue = cap_right!(1, 0x4u64),
    SemPost = cap_right!(1, 0x8u64),
    SemWait = cap_right!(1, 0x10u64),
    Event = cap_right!(1, 0x20u64),
    KqueueEvent = cap_right!(1, 0x40u64),
    Ioctl = cap_right!(1, 0x80u64),
    Ttyhook = cap_right!(1, 0x100u64),
    Pdgetpid = cap_right!(1, 0x200u64),
    Pdwait = cap_right!(1, 0x400u64),
    Pdkill = cap_right!(1, 0x800),
    ExtattrDelete = cap_right!(1, 0x1000u64),
    ExtattrGet = cap_right!(1, 0x2000u64),
    ExtattrList = cap_right!(1, 0x4000u64),
    ExtattrSet = cap_right!(1, 0x8000u64),
    AclCheck = cap_right!(1, 0x10000u64),
    AclDelete = cap_right!(1, 0x20000u64),
    AclGet = cap_right!(1, 0x40000u64),
    AclSet = cap_right!(1, 0x80000u64),
    KqueueChange = cap_right!(1, 0x100000u64),
    Kqueue = right_or!(Right::KqueueEvent, Right::KqueueChange),
    All1 = cap_right!(1, 0x1FFFFFu64),
    Unused122 = cap_right!(1, 0x200000u64),
    Unused157 = cap_right!(1, 0x100000000000000u64),
}

#[derive(Debug, Default)]
pub struct RightsBuilder(u64);

impl RightsBuilder {
    pub fn new(right: Right) -> RightsBuilder {
        RightsBuilder(right as u64)
    }

    pub fn add(&mut self, right: Right) -> &mut RightsBuilder {
        self.0 |= right as u64;
        self
    }

    pub fn finalize(&self) -> CapResult<FileRights> {
        FileRights::new(self.0)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn remove(&mut self, right: Right) -> &mut RightsBuilder {
        self.0 = (self.0 & !(right as u64)) | 0x200000000000000;
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct FileRights(cap_rights_t);

impl FileRights {
    pub fn new(raw_rights: u64) -> CapResult<FileRights> {
        unsafe {
            let mut empty_rights = unsafe { mem::zeroed() };
            let rights_ptr = libc::__cap_rights_init(
                RIGHTS_VERSION,
                &mut empty_rights as *mut cap_rights_t,
                raw_rights,
                0u64,
            );
            if rights_ptr.is_null() {
                Err(CapErr::from(CapErrType::Generic))
            } else {
                let rights = FileRights(empty_rights);
                if rights.is_valid() {
                    Ok(rights)
                } else {
                    Err(CapErr::from(CapErrType::Invalid))
                }
            }
        }
    }

    pub fn from_file<T: AsRawFd>(fd: &T) -> CapResult<FileRights> {
        unsafe {
            let mut empty_rights = unsafe { mem::zeroed() };
            let res = libc::__cap_rights_get(
                RIGHTS_VERSION,
                fd.as_raw_fd(),
                &mut empty_rights as *mut cap_rights_t,
            );
            if res < 0 {
                Err(CapErr::from(CapErrType::Get))
            } else {
                let rights = FileRights(empty_rights);
                if rights.is_valid() {
                    Ok(rights)
                } else {
                    Err(CapErr::from(CapErrType::Invalid))
                }
            }
        }
    }

    pub fn contains(&self, other: &FileRights) -> bool {
        unsafe { libc::cap_rights_contains(&self.0, &other.0) }
    }

    pub fn is_set(&self, raw_rights: Right) -> bool {
        unsafe {
            libc::__cap_rights_is_set(&self.0 as *const cap_rights_t, raw_rights as u64, 0u64)
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { libc::cap_rights_is_valid(&self.0) }
    }

    pub fn merge(&mut self, other: &FileRights) -> CapResult<()> {
        unsafe {
            let result = libc::cap_rights_merge(&mut self.0 as *mut cap_rights_t, &other.0);
            if result.is_null() {
                Err(CapErr::from(CapErrType::Merge))
            } else {
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, other: &FileRights) -> CapResult<()> {
        unsafe {
            let result = libc::cap_rights_remove(&mut self.0 as *mut cap_rights_t, &other.0);
            if result.is_null() {
                Err(CapErr::from(CapErrType::Remove))
            } else {
                Ok(())
            }
        }
    }

    pub fn set(&mut self, raw_rights: Right) -> CapResult<()> {
        unsafe {
            let result =
                libc::__cap_rights_set(&mut self.0 as *mut cap_rights_t, raw_rights as u64, 0u64);
            if result.is_null() {
                Err(CapErr::from(CapErrType::Set))
            } else {
                Ok(())
            }
        }
    }

    pub fn clear(&mut self, raw_rights: Right) -> CapResult<()> {
        unsafe {
            let result =
                libc::__cap_rights_clear(&mut self.0 as *mut cap_rights_t, raw_rights as u64, 0u64);
            if result.is_null() {
                Err(CapErr::from(CapErrType::Clear))
            } else {
                Ok(())
            }
        }
    }
}

impl CapRights for FileRights {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()> {
        unsafe {
            let res = libc::cap_rights_limit(fd.as_raw_fd(), &self.0 as *const cap_rights_t);
            if res < 0 {
                Err(CapErr::from(CapErrType::Limit))
            } else {
                Ok(())
            }
        }
    }
}

#[test]
fn test_macros() {
    assert_eq!(cap_right!(0, 1), 144115188075855873u64);
    assert_eq!(right_or!(Right::Read, Right::Write), 144115188075855875u64);
}
