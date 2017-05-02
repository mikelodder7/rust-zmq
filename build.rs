fn main() {
    for has in ["ipc"].into_iter() {
        println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
    }
}
