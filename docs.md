# Printデバッグしたい
-　手動で実装する必要があるが、`fmt::Debug`が簡潔に実現してくれる

1. デバッグしたい構造体に`#[derive(Debug)]` を追記
2. `println!("{:#?}", )`でプリント

```rust
#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}


let mut uf = UnionFind::new(N);
println!("{:#?}", uf);
```
参考：[デバッグ - Rust By Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/hello/print/print_debug.html)

# 269
## B
### 配列から特定の要素のindexを取得したい

```rust
array.iter().position(|a| *a == b'#').unwrap();
```
