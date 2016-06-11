/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


extern crate capsicum;

mod tests {
    use capsicum::{enter, Right, RightsBuilder, sandboxed};
    use std::fs;
    use std::io::{Read, Write};

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
        let builder = RightsBuilder::new(Right::Read).add(Right::Write);
        let value = ((1u64 << (57 + 0x0u64))) | (0x3u64);
        assert_eq!(value, builder.bits());
    }

    #[test]
    fn test_rights() {
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
        if unsafe { fork() } == 0 {
            enter().expect("cap_enter failed!");
            assert!(sandboxed(), "application isn't sandboxed");
        }
        unsafe {
            wait();
        }
    }
}
