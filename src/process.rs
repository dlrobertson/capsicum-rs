// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;

pub fn enter() -> io::Result<()> {
    if unsafe { cap_enter() } < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn sandboxed() -> bool {
    unsafe { cap_sandboxed() == 1 }
}

pub fn get_mode() -> io::Result<usize> {
    let mut mode = 0;
    unsafe {
        if cap_getmode(&mut mode as *mut usize) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(mode)
}

extern "C" {
    fn cap_enter() -> isize;
    fn cap_sandboxed() -> isize;
    fn cap_getmode(modep: *mut usize) -> isize;
}
