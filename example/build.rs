fn main() {
    // C言語のコードをビルドする設定
    // compile()に渡すライブラリ名は他と重複しないこと(libffi.aとffiは重複する)
    cc::Build::new()
        .file("src/ffi.c")
        .include("src")
        .compile("ffi_c");
    // C++言語のコードをビルドする設定
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++20")
        // .warnings(true)
        // .flag("-Wall")
        // .flag("-Wextra")
        // .flag("-v")
        // .flag("-g")
        .file("src/ffi.cpp")
        .compile("ffi_cpp");
}
