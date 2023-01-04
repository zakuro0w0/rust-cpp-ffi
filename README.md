## 参考にした記事
- [Rust で C と C++ の FFI](https://blog.ojisan.io/rust-ffi-cpp-wakaran/)
- [Rust の Foreign Function Interface (FFI) - Qiita](https://qiita.com/termoshtt/items/0fa9959f9eb64b0907e2)
- [C言語へのFFIを含むRustをWASM化するのは難しすぎる](https://zenn.dev/newgyu/articles/4240df5d2a7d55)
- [C言語へのFFIを含むRustコードをWASMにする（CMakeを添えて）](https://zenn.dev/newgyu/articles/8bff73505c7b35)


## build.rsにリンク設定を書いてFFIを実行する場合
### C言語コードから静的ライブラリを作る
```
gcc -fPIC src/ffi.c -o target/libffi.a
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