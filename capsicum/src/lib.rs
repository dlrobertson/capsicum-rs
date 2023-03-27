// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! ## Entering capability mode
//!
//! ```
//!  use capsicum::{enter, sandboxed};
//!  use std::fs::File;
//!  use std::io::Read;
//!
//!  let mut ok_file = File::open("/etc/passwd").unwrap();
//!  let mut s = String::new();
//!
//!  enter().expect("enter failed!");
//!  assert!(sandboxed(), "application is not sandboxed!");
//!
//!  match File::create("/tmp/cant_touch_this") {
//!      Ok(_) => panic!("application is not properly sandboxed!"),
//!      Err(e) => println!("properly sandboxed: {:?}", e)
//!  }
//!
//!  match ok_file.read_to_string(&mut s) {
//!      Ok(_) => println!("This is okay since we opened the descriptor before sandboxing"),
//!      Err(_) => panic!("application is not properly sandboxed!")
//!  }
//! ```
//!
//! ## Limit capability rights to files
//!
//! ```
//! use capsicum::{CapRights, Right, RightsBuilder};
//! use std::fs::File;
//! use std::io::Read;
//! let mut ok_file = File::open("/etc/passwd").unwrap();
//! let mut s = String::new();
//!
//! let mut builder = RightsBuilder::new(Right::Seek);
//!
//! builder.add(Right::Read);
//!
//! let rights = builder.finalize();
//!
//! rights.limit(&ok_file).unwrap();
//!
//! assert!(ok_file.read_to_string(&mut s).is_ok());
//! ```
#[cfg(feature = "casper")]
#[cfg_attr(docsrs, doc(cfg(feature = "casper")))]
pub mod casper;
mod common;
mod fcntl;
mod ioctl;
mod process;
mod right;
pub mod util;

pub use fcntl::{Fcntl, FcntlRights, FcntlsBuilder};
pub use ioctl::{IoctlRights, IoctlsBuilder};
pub use process::{enter, get_mode, sandboxed};
pub use right::{FileRights, Right, RightsBuilder};

pub use crate::common::CapRights;
