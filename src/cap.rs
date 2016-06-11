#![allow(dead_code)]
#![allow(non_camel_case_types)]

use rights::{RIGHTS_VERSION, Right};
use std::fs::File;
use std::os::unix::io::AsRawFd;

pub struct Rights(cap_rights_t);

impl Rights {
    pub fn new(raw_rights: u64) -> Result<Rights, ()> {
        unsafe {
            let mut empty_rights = cap_rights_t {
                cr_rights: [0; RIGHTS_VERSION + 2],
            };
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

    pub fn contains(&self, other: &Rights) -> bool {
        unsafe { cap_rights_contains(&self.0, &other.0) }
    }

    pub fn is_set(&self, raw_rights: Right) -> bool {
        unsafe {
            __cap_rights_is_set(&self.0 as *const cap_rights_t,
                                raw_rights as u64,
                                0u64)
        }
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
            let result = __cap_rights_set(&mut self.0 as *mut cap_rights_t,
                                          raw_rights as u64,
                                          0u64);
            if result.is_null() {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn clear(&mut self, raw_rights: Right) -> Result<(), ()> {
        unsafe {
            let result = __cap_rights_clear(&mut self.0 as *mut cap_rights_t,
                                            raw_rights as u64,
                                            0u64);
            if result.is_null() {
                Err(())
            } else {
                Ok(())
            }
        }
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

    pub fn add(mut self, right: Right) -> RightsBuilder {
        self.0 |= right as u64;
        self
    }

    pub fn remove(mut self, right: Right) -> RightsBuilder {
        self.0 ^= right as u64;
        self
    }

    pub fn finalize(&self) -> Result<Rights, ()> {
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

#[repr(C)]
pub struct cap_rights_t {
    cr_rights: [u64; RIGHTS_VERSION + 2],
}

type cap_rights = cap_rights_t;

extern {
    fn cap_enter() -> isize;
    fn cap_sandboxed() -> isize;
    fn cap_rights_is_valid(rights: *const cap_rights_t) -> bool;
    fn cap_rights_merge(dst: *mut cap_rights_t, src: *const cap_rights_t) -> *mut cap_rights_t;
    fn cap_rights_remove(dst: *mut cap_rights_t, src: *const cap_rights_t) -> *mut cap_rights_t;
    fn cap_rights_contains(big: *const cap_rights_t, little: *const cap_rights_t) -> bool;
    fn cap_rights_limit(fd: isize, rights: *const cap_rights_t) -> isize;
    fn __cap_rights_init(version: usize, rights: *mut cap_rights_t, raw_rights: u64, sentinel: u64) -> *mut cap_rights_t;
    fn __cap_rights_set(rights: *mut cap_rights_t, raw_rights: u64, sentinel: u64) -> *mut cap_rights_t;
    fn __cap_rights_clear(rights: *mut cap_rights_t, raw_rights: u64, sentinel: u64) -> *mut cap_rights_t;
    fn __cap_rights_is_set(rights: *const cap_rights_t, raw_rights: u64, sentinel: u64) -> bool;
}

#[cfg(test)]
mod tests {
    extern {
        fn fork() -> isize;
        fn wait() -> isize;
    }

    #[test]
    fn test_rights_builer() {
        use rights::Right;
        use super::RightsBuilder;
        let builder = RightsBuilder::new(Right::Read).add(Right::Write);
        let value = ((1u64 << (57 + 0x0u64))) | (0x3u64);
        assert_eq!(value, builder.bits());
    }

    #[test]
    fn test_rights() {
        use std::fs;
        use rights::Right;
        use std::io::Write;
        use std::io::Read;
        use super::RightsBuilder;

        let filepath = "/tmp/capsicum_test";

        let mut file = fs::File::create(filepath).unwrap();

        let mut rights = match RightsBuilder::new(Right::Null).finalize() {
            Ok(rights) => rights,
            _ => panic!("Error in creation of rights")
        };

        let to_merge = RightsBuilder::new(Right::Write).finalize().unwrap();

        rights.merge(&to_merge).unwrap();

        rights.set(Right::Read).unwrap();

        rights.clear(Right::Write).unwrap();

        assert!(rights.is_set(Right::Read));

        assert!(!rights.contains(&to_merge));

        rights.limit(&file);

        let c_string = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20,
                        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21];

        // Write should be limitted
        if let Ok(_) = file.write_all(&c_string) {
            fs::remove_file(filepath).unwrap();
            panic!("Rights did not correctly limit write");
        }

        file = fs::File::open(filepath).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // Nothing has been written to the file
        assert_eq!("", s);

        // Cleanup
        fs::remove_file(filepath).unwrap();
    }

    #[test]
    fn test_enter() {
        use super::{enter, sandboxed};
        if unsafe { fork() } == 0 {
            enter().expect("cap_enter failed!");
            assert!(sandboxed(), "application isn't sandboxed");
        }
        unsafe {
            wait();
        }
    }
}
