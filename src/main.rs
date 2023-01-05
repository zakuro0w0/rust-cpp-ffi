// use std::os::raw::c_int;

// C/C++で実装した関数を宣言する(名前が一致していること)
extern "C" {
    // C++で実装した関数
    // C++のintはi32に相当する
    fn hello_world_cpp() -> i32;
    // Cで実装した関数
    fn hello_world_c() -> i32;
}

fn main() {
    println!("Hello, world!");
    // FFIで関数を呼び出す場合はunsafeブロックで囲む必要がある
    // Rustコンパイラはunsafeブロックのメモリ安全性を保障しない
    unsafe {
        println!("return C++ : {}", hello_world_cpp());
        println!("return C : {}", hello_world_c());
    }
}
