use std::ffi::{CStr, CString};

// C/C++で実装した関数を宣言する(名前が一致していること)
extern "C" {
    // C++で実装した関数
    // C++のintはi32に相当する
    fn hello_world_cpp() -> i32;
    // Cで実装した関数
    fn hello_world_c() -> i32;
}

// 自前で定義したFFIの呼び出し
fn self_ffi() {
    // FFIで関数を呼び出す場合はunsafeブロックで囲む必要がある
    // Rustコンパイラはunsafeブロックのメモリ安全性を保障しない
    unsafe {
        println!("return C++ : {}", hello_world_cpp());
        println!("return C : {}", hello_world_c());
    }
}

// ffi-sysでbindgenを使って定義したFFIの呼び出し
fn bindgen_ffi() {
    let c_string = CString::new("Hello FFI").unwrap();
    unsafe {
        // ffi-sys/Cargo.tomlで
        ffi_sys::print_str(c_string.as_ptr());
    }
}

fn main() {
    println!("Hello, world!");
    self_ffi();
    bindgen_ffi();
}
