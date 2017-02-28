// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// ## Entering capability mode
///
/// ```ignore
///  use capsicum::{enter, sandboxed};
///  use std::fs::File;
///  use std::io::Read;
///
///  let mut ok_file = File::open("/tmp/foo").unwrap();
///  let mut s = String::new();
///
///  enter().expect("enter failed!");
///  assert!(sandboxed(), "application is not sandboxed!");
///
///  match File::create("/tmp/cant_touch_this") {
///      Ok(_) => panic!("application is not properly sandboxed!"),
///      Err(e) => println!("properly sandboxed: {:?}", e)
///  }
///
///  match ok_file.read_to_string(&mut s) {
///      Ok(_) => println!("This is okay since we opened the descriptor before sandboxing"),
///      Err(_) => panic!("application is not properly sandboxed!")
///  }
/// ```
///
/// ## Limit capability rights to files
///
/// ```ignore
/// use capsicum::{CapRights, Right, RightsBuilder};
/// use std::fs::File;
/// use std::io::Read;

/// let x = rand::random::<bool>();
///
/// let mut ok_file = File::open("/tmp/foo").unwrap();
/// let mut s = String::new();
///
/// let mut builder = RightsBuilder::new(Right::Seek);
///
/// if x {
///     builder.add(Right::Read);
/// }

/// let rights = builder.finalize().unwrap();

/// rights.limit(&ok_file).unwrap();
///
/// match ok_file.read_to_string(&mut s) {
///     Ok(_) if x => println!("Allowed reading: x = {} ", x),
///     Err(_) if !x => println!("Did not allow reading: x = {}", x),
///     _ => panic!("Not properly sandboxed"),
/// }
/// ```

extern crate libc;

mod common;
mod fcntl;
mod ioctl;
mod process;
mod right;
pub mod util;

pub use common::{CapResult, CapErr, CapRights};
pub use fcntl::{Fcntl, FcntlRights, FcntlsBuilder};
pub use ioctl::{IoctlRights, IoctlsBuilder};
pub use process::{get_mode, enter, sandboxed};
pub use right::{Right, FileRights, RightsBuilder};
