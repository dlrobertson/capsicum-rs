#[cfg(target_os = "freebsd")]
fn freebsd_nop() {
}

#[cfg(not(target_os = "freebsd"))]
fn freebsd_nop() {
    panic!("This is a FreeBSD only crate. It will not compile on other OSes.");
}

fn main() {
    freebsd_nop();
}
