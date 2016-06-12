/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


extern crate capsicum;

mod tests {
    use capsicum::{enter, Right, Rights, RightsBuilder, Ioctls, IoctlsBuilder, sandboxed};
    use std::fs;
    use std::io::{Read, Write};

    const TMPFILE1: &'static str = "/tmp/foo";
    const TMPFILE2: &'static str = "/tmp/bar";
    const TMPFILE3: &'static str = "/tmp/baz";

    extern {
        fn fork() -> isize;
        fn wait() -> isize;
    }

    #[test]
    fn test_rights_right() {
        assert_eq!(144115188075855873u64, Right::Read as u64);
    }

    #[test]
    fn test_rights_builer() {
        let mut builder = RightsBuilder::new(Right::Read);
        builder.add(Right::Lookup).add(Right::AclSet)
                                  .add(Right::AclSet)
                                  .add(Right::AclGet)
                                  .add(Right::Write)
                                  .remove(Right::Lookup)
                                  .remove(Right::AclGet);
        assert_eq!(144115188076380163, builder.bits());
    }

    #[test]
    fn test_rights() {
        let mut file = fs::File::create(TMPFILE1).unwrap();

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

        let from_file = Rights::from_file(&file).unwrap();

        assert_eq!(rights, from_file);

        let c_string = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20,
                        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21];

        // Write should be limitted
        if let Ok(_) = file.write_all(&c_string) {
            fs::remove_file(TMPFILE1).unwrap();
            panic!("Rights did not correctly limit write");
        }

        file = fs::File::open(TMPFILE1).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // Nothing has been written to the file
        assert_eq!("", s);

        fs::remove_file(TMPFILE1).unwrap();
    }

    #[test]
    fn test_enter() {
        if unsafe { fork() } == 0 {
            fs::File::create(TMPFILE2).unwrap();
            let mut ok_file = fs::File::open(TMPFILE2).unwrap();

            enter().expect("cap_enter failed!");
            assert!(sandboxed(), "application is not properly sandboxed");

            if let Ok(_) = fs::File::open(TMPFILE1) {
                panic!("application is not properly sandboxed!");
            }

            let mut s = String::new();
            if let Err(_) = ok_file.read_to_string(&mut s) {
                panic!("application is not properly sandboxed!");
            }
        }
        unsafe {
            wait();
        }
        fs::remove_file(TMPFILE2).unwrap();
    }

    #[test]
    fn test_ioctl() {
        let file = fs::File::create(TMPFILE3).unwrap();
        let ioctls = IoctlsBuilder::new(9223372036854775807).finalize();
        let x = ioctls.limit(&file);
        if x < 0 {
            panic!("failed!");
        }
        let _ = Ioctls::from_file(&file, 10);
    }
}
