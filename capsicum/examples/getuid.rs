//! A toy Casper service that provides `getuid()`.

use std::{ffi::CStr, io};

use capsicum::casper::{self, Casper, NvError, NvFlag, NvList, ServiceRegisterFlags};
use cstr::cstr;
use libc::uid_t;

/// The Capser `uid` helper process.
struct CapUid {}
impl casper::Service for CapUid {
    const SERVICE_NAME: &'static CStr = cstr!("getuid");

    fn cmd(
        cmd: &str,
        limits: Option<&NvList>,
        _nvin: Option<&mut NvList>,
        nvout: &mut NvList,
    ) -> io::Result<()> {
        assert_eq!(cmd, "getuid");

        // First check if we've been limited
        if limits.is_some() {
            return Err(io::Error::from_raw_os_error(libc::ENOTCAPABLE));
        }

        // This C function is always safe
        let uid = unsafe { libc::getuid() };
        nvout.insert_number("uid", uid).unwrap();
        Ok(())
    }

    fn limit(_old: Option<&NvList>, _new: Option<&NvList>) -> io::Result<()> {
        // In our toy example, we don't need to do anything here.  Instead,
        // we'll check the limits on every call to `cmd`
        Ok(())
    }
}
casper::service!(
    /// A connection to the Casper `uid` helper.
    pub CapUid, CapAgent, uid, ServiceRegisterFlags::NONE
);

impl CapAgent {
    /// Retrieve the process's uid.
    // Or really, the uid of the Casper helper process.  This is dumb, because
    // `getuid` works fine in capability mode, but it's a nice simple demo.
    pub fn uid(&mut self) -> io::Result<uid_t> {
        let mut invl = NvList::new(NvFlag::None).unwrap();
        invl.insert_string("cmd", "getuid").unwrap();
        let onvl = self.xfer_nvlist(invl)?;
        match onvl.get_number("uid") {
            Ok(Some(uid)) => Ok(uid as uid_t),
            Ok(None) => panic!("zygote did not return the expected value"),
            Err(NvError::NativeError(e)) => Err(io::Error::from_raw_os_error(e)),
            Err(NvError::Io(e)) => Err(e),
            _ => unimplemented!(),
        }
    }

    /// Limit the ability to use the [`uid`] method. After calling this, the
    /// Casper help will refuse future `uid` commands.
    pub fn limit_uid(&mut self) -> io::Result<()> {
        let limits = self.limit_get()?;
        // In our toy example, the limits should always be empty
        assert!(limits.is_empty());

        // A more complete service would add fields to the limits nvlist at this
        // point.
        self.limit_set(limits)
    }
}

fn main() {
    // Safe because we're still single-threaded
    let casper = unsafe { Casper::new().unwrap() };
    capsicum::enter().unwrap();

    let mut cap_uid = casper.uid().unwrap();
    println!("UID is {:?}", cap_uid.uid().unwrap());

    // Now we'll limit the service, and the uid command should fail
    cap_uid.limit_uid().unwrap();
    assert_eq!(
        libc::ENOTCAPABLE,
        cap_uid.uid().unwrap_err().raw_os_error().unwrap()
    );
}
