# capsicum-rs

## Contain the awesome!

Rust bindings for the FreeBSD [capsicum](https://www.freebsd.org/cgi/man.cgi?query=capsicum)
framework for OS capability and sandboxing

## Prerequisites

[Rust](https://www.rust-lang.org/), [Cargo](https://crates.io/), and [FreeBSD](https://www.freebsd.org/).

**Note:** This currently only compiles on FreeBSD

## Getting Started

### Get the code

```
git clone https://github.com/danlrobertson/capsicum-rs
cd capsicum-rs
cargo build
```

### Writing code using `capsicum-rs`

#### Entering capability mode

```
 use capsicum::{enter, sandboxed};
 use std::fs::File;
 use std::io::Read;

 let mut ok_file = File::open("/tmp/foo").unwrap();
 let mut s = String::new();

 enter().expect("enter failed!");
 assert!(sandboxed(), "application is not sandboxed!");

 match File::create("/tmp/cant_touch_this") {
     Ok(_) => panic!("application is not properly sandboxed!"),
     Err(e) => println!("properly sandboxed: {:?}", e)
 }

 match ok_file.read_to_string(&mut s) {
     Ok(_) => println!("This is okay since we opened the descriptor before sandboxing"),
     Err(_) => panic!("application is not properly sandboxed!")
 }
```

#### Limit capability rights to files

```
 use capsicum::{Right, RightsBuilder};
 use std::fs::{self, File};

 let x = rand::random::<u8>();

 let mut ok_file = File::open("/tmp/foo").unwrap();
 let mut s = String::new();

 let mut builder = RightsBuilder::new(Right::Seek);

 if if x < 42 {
     builder.add(Right::Read);
 }

 match ok_file.read_to_string(&mut s) {
     Ok(_) if other_value => println!("Since other value is true we allowed reading"),
     Err(_) if !other_value => panic!("Since other value is false we did not allow reading"),
     _ => panic!("Application is not properly sandboxed!")
 }
```
