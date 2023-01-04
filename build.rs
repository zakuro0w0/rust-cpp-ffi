extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/ffi.c")
        .include("src")
        .compile("ffi");
}
