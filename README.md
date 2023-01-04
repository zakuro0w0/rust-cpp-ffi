
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