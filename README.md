Rust ZeroMQ mobile bindings.

[![Travis Build Status](https://travis-ci.org/erickt/rust-zmq.png?branch=master)](https://travis-ci.org/erickt/rust-zmq)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/xhytsx4jwyb9qk7m?svg=true)](https://ci.appveyor.com/project/erickt/rust-zmq)
[![Coverage Status](https://coveralls.io/repos/erickt/erickt-zmq/badge.svg?branch=master)](https://coveralls.io/r/erickt/erickt-zmq?branch=master)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](http://meritbadge.herokuapp.com/zmq)](https://crates.io/crates/zmq)
[![docs](https://docs.rs/zmq/badge.svg)](https://docs.rs/zmq)

[Documentation](https://docs.rs/crate/zmq/)

[Release Notes](https://github.com/erickt/rust-zmq/tree/master/NEWS.md)

rust-zmq-mobile is intended to be built for environments where zmq
isn't or can't be loaded by the OS such as iOS or Android when rust-zmq
would fail when building with cargo.

# Installation

Currently, rust-zmq-mobile requires ZeroMQ 3.2 or newer. For example, on
recent Debian-based distributions, you can use the following command
to get the prerequiste headers and library installed:

    apt install libzmq3-dev

If your OS of choice does not provide packages of a new-enough libzmq,
you will first have to install it from source; see
<https://github.com/zeromq/libzmq/releases> or build it for your intended
target OS. ZeroMQ can be built for


rust-zmq-mobile uses [cargo](https://crates.io) to install. Users should add this to
their `Cargo.toml` file:

    [dependencies]
    zmq-mobile = "0.1"

The environment variables `LIBZMQ_PREFIX` (or alternatively, `LIBZMQ_LIB_DIR` and
`LIBZMQ_INCLUDE_DIR`) must be defined to dynamically or statically link. If
`LIBZMQ_STATIC`=1 is specified, ZeroMQ libraries will be statically linked rather
than dynamically linked.

To build zmq for Android see [here](https://github.com/mikelodder7/android-building/tree/master/x86/zeromq)

# Usage

`rust-zmq-mobile` is a pretty straight forward port of the C API into Rust:

```rust
extern crate zmq-mobile;

fn main() {
    let ctx = zmq-mobile::Context::new();

    let mut socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send_str("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/mikelodder7/rust-zmq-mobile/tree/master/examples.

# Contributing

Install for contributing to rust-zmq-mobile:

    % git clone https://github.com/mikelodder7/rust-zmq-mobile
    % cd rust-zmq-mobile
    % cargo build

Note that the `master` branch is currently in API-breaking mode while
we try to make the API more ergomic and flexible for the `0.1` release
series.

__This means that pull requests (e.g. bugfixes), which do not need to
break API should be submitted for the `release/v0.8` branch__. This
also applies to new features, if they can be implemented in an
API-compatible way, the pull request should also aim for
`release/v0.1`. Please submit an issue for missing features before you
start coding, so the suitable branch and other potential questions can
be clarified up-front.

The reason for using branches, and thus increasing overhead a bit for
all involved, is that it's not yet clear how long it will take to
reach a point in `master` that we feel comfortable with releasing as
0.1.0, as we'd like to have the core part of the API more-or-less
fixed by then. Using the `release/v0.8` branch we can deliver bugfixes
and smaller features in the meantime without forcing users to follow
master's changing API and to continuously adjust their code to API
changes.
