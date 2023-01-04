// C/C++で実装した関数を宣言する(名前が一致していること)
extern "C" {
    // C++で実装した関数
    fn hello_world_cpp();
    // Cで実装した関数
    fn hello_world_c();
}

fn main() {
    println!("Hello, world!");
    // FFIで関数を呼び出す場合はunsafeブロックで囲む必要がある
    // Rustコンパイラはunsafeブロックのメモリ安全性を保障しない
    unsafe {
        hello_world_cpp();
        hello_world_c();
    }
}
