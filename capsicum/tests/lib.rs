/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![cfg_attr(nightly, feature(panic_always_abort))]

/// Switch the panic handler to SIGABRT, since stack unwinding can't be safely
/// done after a fork.
#[cfg(nightly)]
fn always_abort() {
    std::panic::always_abort();
}
#[cfg(not(nightly))]
fn always_abort() {
    // Nothing we can do.
}

mod base {
    use std::{
        fs,
        io::{Read, Write},
    };

    use capsicum::{
        enter,
        sandboxed,
        CapRights,
        Fcntl,
        FcntlRights,
        FcntlsBuilder,
        FileRights,
        IoctlRights,
        IoctlsBuilder,
        Right,
        RightsBuilder,
    };
    use nix::{
        sys::wait::{waitpid, WaitStatus},
        unistd::{fork, ForkResult},
    };
    use tempfile::{tempfile, NamedTempFile};

    use super::*;

    #[test]
    fn test_rights_right() {
        assert_eq!(144115188075855873u64, Right::Read as u64);
    }

    #[test]
    fn test_rights_builer() {
        let mut builder = RightsBuilder::new(Right::Read);
        let builder = builder.allow(Right::Lookup);
        let builder = builder.allow(Right::AclSet);
        let builder = builder.allow(Right::AclSet);
        let builder = builder.allow(Right::AclGet);
        let builder = builder.allow(Right::Write);
        let builder = builder.deny(Right::Lookup);
        let builder = builder.deny(Right::AclGet);
        assert_eq!(144115188076380163, builder.raw());
    }

    #[test]
    fn test_rights() {
        let mut file = NamedTempFile::new().unwrap();

        let mut rights = RightsBuilder::new(Right::Null).finalize();

        let to_merge = RightsBuilder::new(Right::Write).finalize();

        rights.merge(&to_merge).unwrap();

        rights.allow(Right::Read).unwrap();

        rights.deny(Right::Write).unwrap();

        assert!(rights.is_set(Right::Read));

        assert!(!rights.contains(&to_merge));

        rights.limit(&file).unwrap();

        let from_file = FileRights::from_file(&file).unwrap();

        assert_eq!(rights, from_file);

        let c_string = [
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x00,
        ];

        // Write should be limitted
        if file.write_all(&c_string).is_ok() {
            panic!("Rights did not correctly limit write");
        }

        let mut s = String::new();
        let mut rfile = fs::File::open(file.path()).unwrap();
        rfile.read_to_string(&mut s).unwrap();

        // Nothing has been written to the file
        assert_eq!("", s);
    }

    #[test]
    fn test_enter() {
        let mut file = NamedTempFile::new().unwrap();
        match unsafe { fork() }.unwrap() {
            ForkResult::Child => {
                always_abort();
                enter().expect("cap_enter failed!");
                assert!(sandboxed(), "application is not properly sandboxed");

                if fs::File::open(file.path()).is_ok() {
                    panic!("application is not properly sandboxed!");
                }

                let mut s = String::new();
                if file.read_to_string(&mut s).is_err() {
                    panic!("application is not properly sandboxed!");
                }
                unsafe { libc::_exit(0) };
            }
            ForkResult::Parent { child } => {
                let cstat = waitpid(child, None).unwrap();
                assert!(matches!(cstat, WaitStatus::Exited(_, 0)));
            }
        }
    }

    #[test]
    fn test_ioctl() {
        let file = tempfile().unwrap();
        let ioctls = IoctlsBuilder::new().allow(1).finalize();
        ioctls.limit(&file).unwrap();
        let limited = IoctlRights::from_file(&file, 10).unwrap();
        assert_eq!(ioctls, limited);
    }

    // https://github.com/dlrobertson/capsicum-rs/issues/5
    #[test]
    fn test_ioctl_unlimited() {
        let file = tempfile().unwrap();
        let limited = IoctlRights::from_file(&file, 10).unwrap();
        assert!(matches!(limited, IoctlRights::Unlimited));
    }

    #[test]
    fn test_fcntl() {
        let file = tempfile().unwrap();
        let fcntls = FcntlsBuilder::new()
            .allow(Fcntl::GetFL)
            .allow(Fcntl::GetOwn)
            .finalize();
        fcntls.limit(&file).unwrap();
        let new_fcntls = FcntlRights::from_file(&file).unwrap();
        assert_eq!(new_fcntls, fcntls);
    }
}

mod util {
    use std::fs;

    use capsicum::{self, CapRights, Right, RightsBuilder};
    use nix::{
        sys::wait::{waitpid, WaitStatus},
        unistd::{fork, ForkResult},
    };
    use tempfile::tempdir;

    use super::*;

    /// We can lookup paths in a directory with Right::Lookup
    #[test]
    #[allow(deprecated)]
    fn test_basic_dir_ok() {
        let tdir = tempdir().unwrap();
        let dir = capsicum::util::Directory::new(tdir.path()).unwrap();
        let fname = "foo";
        fs::File::create(tdir.path().join(fname)).unwrap();
        let rights = RightsBuilder::new(Right::Read)
            .allow(Right::Lookup)
            .finalize();
        rights.limit(&dir).unwrap();
        match unsafe { fork() }.unwrap() {
            ForkResult::Child => {
                always_abort();
                capsicum::enter().unwrap();
                let _ = dir.open_file(fname, 0, None).unwrap();
                unsafe { libc::_exit(0) };
            }
            ForkResult::Parent { child } => {
                let cstat = waitpid(child, None).unwrap();
                assert!(matches!(cstat, WaitStatus::Exited(_, 0)));
            }
        }
    }

    /// Without Right::Lookup, looking up paths in a directory is not allowed
    #[test]
    #[allow(deprecated)]
    fn test_basic_dir_err() {
        let tdir = tempdir().unwrap();
        let dir = capsicum::util::Directory::new(tdir.path()).unwrap();
        let fname = "foo";
        fs::File::create(tdir.path().join(fname)).unwrap();
        let rights = RightsBuilder::new(Right::Read).finalize();
        rights.limit(&dir).unwrap();
        match unsafe { fork() }.unwrap() {
            ForkResult::Child => {
                always_abort();
                capsicum::enter().unwrap();
                let e = dir.open_file(fname, 0, None).unwrap_err();
                // The OS should return ENOTCAPABLE, but std::io::ErrorKind
                // doesn't have a kind for that.
                if e.raw_os_error() == Some(libc::ENOTCAPABLE) {
                    unsafe { libc::_exit(0) }
                } else {
                    unsafe { libc::_exit(1) }
                }
            }
            ForkResult::Parent { child } => {
                let cstat = waitpid(child, None).unwrap();
                assert!(matches!(cstat, WaitStatus::Exited(_, 0)));
            }
        }
    }
}
