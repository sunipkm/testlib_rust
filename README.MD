# Example of Rust bindings from C Library
C library: [`testlib_c`](https://github.com/sunipkm/testlib_c).
<br>
Install `cc`, `enum_primitive`, `libc`, `pkg-config` (`cargo add`).

Further, install `bindgen`, `bindgen-cli` (`cargo install`). 

## Steps
1. Install `bindgen` and `bindgen-cli` using `cargo install`.
2. Use `bindgen` on command line to generate the bindings:
```sh
$ bindgen /path/to/testlib_c/include/testlib.h -o src/bindings.rs
```
3. (Optional) fix the `enum` types in `bindings.rs`.
4. Import `bindings.rs` into `lib.rs` and wrap away.