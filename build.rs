use std::env;

fn main() {
    let zmq_has = match env::var("ZMQ_HAS") {
        Ok(value) => value,
        Err(..) => panic!("Please set the ZMQ_HAS environment variable (e.g. \"ipc,pgm,tipc,norm,curve,gssapi\")")
    };

    println!("rerun-if-env-changed=ZMQ_HAS");

    for has in zmq_has.split(",") {
        println!("cargo:rustc-cfg=ZMQ_HAS_{}=\"1\"", has.to_uppercase());
    }
}
