// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{ffi, io, os::unix::io::AsRawFd};

pub enum CapErrType {
    Clear,
    Generic,
    Get,
    Invalid,
    Limit,
    Merge,
    Remove,
    Set,
}

#[derive(Debug)]
pub enum CapErr {
    Clear(io::Error),
    Generic(io::Error),
    Get(io::Error),
    Invalid(io::Error),
    Limit(io::Error),
    Merge(io::Error),
    Nul(ffi::NulError),
    Remove(io::Error),
    Set(io::Error),
}

impl From<CapErrType> for CapErr {
    fn from(other: CapErrType) -> CapErr {
        match other {
            CapErrType::Clear => CapErr::Clear(io::Error::last_os_error()),
            CapErrType::Generic => CapErr::Generic(io::Error::last_os_error()),
            CapErrType::Get => CapErr::Get(io::Error::last_os_error()),
            CapErrType::Invalid => CapErr::Invalid(io::Error::last_os_error()),
            CapErrType::Limit => CapErr::Limit(io::Error::last_os_error()),
            CapErrType::Merge => CapErr::Merge(io::Error::last_os_error()),
            CapErrType::Remove => CapErr::Remove(io::Error::last_os_error()),
            CapErrType::Set => CapErr::Set(io::Error::last_os_error()),
        }
    }
}

pub type CapResult<T> = Result<T, CapErr>;

pub trait CapRights: Sized {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()>;
}
