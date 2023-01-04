## 参考にした記事
- [Rust で C と C++ の FFI](https://blog.ojisan.io/rust-ffi-cpp-wakaran/)
- [Rust の Foreign Function Interface (FFI) - Qiita](https://qiita.com/termoshtt/items/0fa9959f9eb64b0907e2)
- [C言語へのFFIを含むRustをWASM化するのは難しすぎる](https://zenn.dev/newgyu/articles/4240df5d2a7d55)
- [C言語へのFFIを含むRustコードをWASMにする（CMakeを添えて）](https://zenn.dev/newgyu/articles/8bff73505c7b35)

## cc crateを使ってC言語ライブラリのビルドを自動化する場合
- build.rsにリンク設定を書いたり、直接rustcコマンドを実行する場合は事前にC言語ライブラリをビルドしておかなければならない
- 面倒なのでcc crateを使ってC言語ソースコードの場所と名前を指定するだけでOKにしておきたい

### cc crateをCargo.tomlに追加する
- [cc crate](https://crates.io/crates/cc)

```toml
[build-dependencies]
cc = "1.0"
```

### build.rsでC言語コードの場所などを設定する
```rs
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/ffi.c")
        .include("src")
        .compile("ffi");
}
```

### 実行する(簡単)
- これだけでC言語コードからライブラリの生成、リンクまでを自動化できた

```
cargo run
```


## build.rsにリンク設定を書いてFFIを実行する場合

### 事前にC言語コードから静的ライブラリを作る
```
gcc -fPIC src/ffi.c -o target/libffi.a
```

### build.rsにC言語ライブラリのリンク設定を書く
```rs
use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/target/", project_dir);
    println!("cargo:rustc-link-lib=ffi");
```


### Rustコードのコンパイル + Cコードのリンク + バイナリ実行
```
cargo run
```

## コマンドでFFIを実行する場合
### C言語コードから静的ライブラリを作る
```
gcc -fPIC src/ffi.c -o target/libffi.a
```

### C言語ライブラリのリンクを指定しつつRustコードをビルド

```
rustc src/main.rs -Ltarget -lffi
```

### Rustコードからビルドしたバイナリを実行する
- これでC言語側で実装した関数の呼び出しが確認できる

```
./main
```