extern crate version_check as rustc;

#[cfg(target_os = "freebsd")]
fn freebsd_nop() {
}

#[cfg(not(target_os = "freebsd"))]
fn freebsd_nop() {
    panic!("This is a FreeBSD only crate. It will not compile on other OSes.");
}

fn main() {
    freebsd_nop();
    if rustc::is_feature_flaggable() == Some(true) {
        println!("cargo:rustc-cfg=nightly")
    }
}
