# RustでAtcoderの精進するためのリポジトリ
## なぜRust??
- これを読んでからずっとRustやりたい〜と考えていた
[六本木ではたらくソフトウェアエンジニアへのよくある質問とその答え (FAQ) (2015 - 2017) - hayato](https://hayatoito.github.io/2017/faq/#rust-is-hard:~:text=%E7%94%9F%E7%94%A3%E7%9A%84%E3%81%A7%E3%81%99%E3%80%82-,Rust%20%E3%81%A3%E3%81%A6%E5%AD%A6%E7%BF%92%E3%82%B3%E3%82%B9%E3%83%88%E3%81%8C%E9%AB%98%E3%81%84%E3%82%93%E3%81%A7%E3%81%99%E3%82%88%E3%81%AD%EF%BC%9F%20%E9%9B%A3%E3%81%97%E3%81%9D%E3%81%86%E3%80%82,-Rust%20%E3%81%AF%E5%AD%A6%E7%BF%92)

## なぜAtcoderやるの？
- アルゴリズムの勉強したかった
- 過去にやってみたが、身に付かなかった&それほど精進したとは思えないので『精一杯やった』と言えるくらいまでは精進したい

# 設定
- `cargo-atcoder`を利用して、test, submitをしています
- なぜか、`cargo-atcoder.toml`は `~/~/Library/Application Support/cargo-atcoder.toml`にありました

### 参考記事
[cargo-atcoder を使って Rust での AtCoder ライフを超快適にする - Qiita](https://qiita.com/maguro_tuna/items/316068eeb8c5b9b31ed8#%E6%8F%90%E5%87%BA%E3%81%99%E3%82%8B)


現状の設定
```toml
[atcoder]
submit_via_binary = false # submit via binary by default
use_cross = false         # use `cross` instead of `cargo` when generating binaries
binary_column = 80        # maximum column number of generated binary (0 for no wrapping)
update_interval = 1000    # interval time of fetching result (ms)
strip_path = "strip"      # specify `strip` command path. NOTE: if you use macOS, you have to install GNU strip and specify its path here.

[profile]
# target to use to generate binary
target = "x86_64-unknown-linux-musl"

[profile.release]
lto = true
panic = 'abort'

# dependencies added to new project
[dependencies]
proconio = "*"
# competitive = { git = "https://github.com/tanakh/competitive-rs.git" }

[project]
# to create `rust-toolchain` file, uncomment this line.
# rustc_version = "1.15.1"

# source code template
template = """
use proconio::input;

fn main() {
    unimplemented!();
}
"""
```

## 使い方
- `cargo atcoder new abc165`
- cd abc165
- Cargo.tomlに以下を追記
	```toml
	[[bin]]
	name = "a"
	path = "src/bin/a.rs"

	[[bin]]
	name = "b"
	path = "src/bin/b.rs"

	[[bin]]
	name = "c"
	path = "src/bin/c.rs"

	[[bin]]
	name = "d"
	path = "src/bin/d.rs"

	[[bin]]
	name = "e"
	path = "src/bin/e.rs"
	```
	- これで、手元で `cargo run --bin a`とすれば`a.rs`を実行できます
- `cargo atcoder test a`
- `cargo atcoder submit a`

### 個人的な設定
`cargo atcoder`が長くて打つのが大変なのでaliasを貼っています

```bash
alias ca='cargo atcoder'
```