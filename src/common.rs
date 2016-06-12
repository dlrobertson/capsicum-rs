// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub enum CapErr {
    Clear(String),
    Generic(String),
    Get(String),
    Invalid(String),
    Limit(String),
    Merge(String),
    Remove(String),
    Set(String)
}

pub type CapResult<T> = Result<T, CapErr>;

pub trait CapRights: Sized {
    fn limit<T: AsRawFd>(&self, fd: &T) -> CapResult<()>;
}
