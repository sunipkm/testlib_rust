fn main() {
    println!("cargo:rustc-link-search={}", "~/testlib_c");
    println!("cargo:rustc-link-lib=dylib=testlib");
  }