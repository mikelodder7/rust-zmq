use std::collections::HashSet;
use std::env;
use std::path::Path;

fn prefix_dir(env_name: &str, dir: &str) -> String {
    match env::var(env_name) {
        Ok(value) => value,
        Err(..) => match env::var("LIBZMQ_PREFIX") {
            Ok(prefix) => Path::new(&prefix).join(dir).to_string_lossy().into_owned(),
            Err(..) => panic!("Please set the {} or LIBZMQ_PREFIX environment variable", env_name)
        }
    }
}

fn determine_mode(libdir: &Path) -> &'static str {
    let kind = env::var("LIBZMQ_STATIC");
    match kind.as_ref() {
        Ok(value) => match value.as_str() {
            "0" => return "dylib",
            _ => return "static"
        },
        Err(..) => {} 
    }

    // Next, see what files we actually have to link against, and see what our
    // possibilities even are.
    let files = libdir
        .read_dir()
        .unwrap()
        .map(|e| e.unwrap())
        .map(|e| e.file_name())
        .filter_map(|e| e.into_string().ok())
        .collect::<HashSet<_>>();
    let can_static = files.contains("libzmq.a") || files.contains("zmq.lib");
    let can_dylib = files.contains("libzmq.so") || files.contains("zmq.dll") || files.contains("libzmq.dylib");
    match (can_static, can_dylib) {
        (true, false) => return "static",
        (false, true) => return "dylib",
        (false, false) => {
            panic!(
                "ZeroMQ libdir at `{}` does not contain the required files \
                 to either statically or dynamically link ZeroMQ",
                libdir.display()
            );
        }
        (true, true) => {}
    }

    // Ok, we've got not explicit preference and can *either* link statically or
    // link dynamically. In the interest of "security upgrades" and/or "best
    // practices with security libs", let's link dynamically.
    "dylib"
}

fn main() {
    let lib_path = prefix_dir("LIBZMQ_LIB_DIR", "lib");
    let include = prefix_dir("LIBZMQ_INCLUDE_DIR", "include");
    let link = determine_mode(Path::new(&lib_path));

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:include={}", include);
    println!("cargo:rustc-link-lib={}=zmq", link);
}
