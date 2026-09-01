# test cross compile on android

> This CI works.

cross compile: `host != target`

rust sets android as tier 2 without host tools, we can compile for android by cross compile.

some crates with name suffix `-sys` use ffi lib, is not friendly, avoid them.
e.g. use rustls than native-tls.

github runners are x86-linux, test with host toolchain, build with android, `aarch64-linux-android`.

