# 解説リポジトリ
[E869120/kyopro-tessoku: 拙著『競技プログラミングの鉄則』（2022/9/16 発売）の GitHub ページです。演習問題の解答や、C++ 以外のソースコードなどが掲載されています。ぜひご活用ください。](https://github.com/E869120/kyopro-tessoku)

# Rubyとの比較
[Ruby脳のためのRust配列系メソッドまとめ](https://zenn.dev/megeton/articles/fb6266bcb6aa1b)

# n進数に変換する

```rust
pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>
```
Converts a string slice in a given base to an integer.

The string is expected to be an optional + or - sign followed by digits. Leading and trailing whitespace represent an error. Digits are a subset of these characters, depending on radix:

0-9
a-z
A-Z
Panics
This function panics if radix is not in the range from 2 to 36.

### Examples
Basic usage:

```rust
assert_eq!(i32::from_str_radix("A", 16), Ok(10));
```

# permutation
[Permutations with replacement in rust? - Stack Overflow](https://stackoverflow.com/questions/71420176/permutations-with-replacement-in-rust)

# Rust document
[Rustのドキュメンテーションコメントについて学ぶ - Qiita](https://qiita.com/simonritchie/items/87d3743e138763ff3e85)

# setを使いたい
[Rust、こんなときはこう書こう - Qiita](https://qiita.com/yasuo-ozu/items/e2e5a960d5b78cda02ef#%E9%87%8D%E8%A4%87%E3%82%92%E5%8F%96%E3%82%8A%E9%99%A4%E3%81%8F)

```rust
fn all_sum(X: Vec<usize>, Y: Vec<usize>) -> Vec<usize> {
    let mut Z: Vec<usize> = vec![0; 0];
    for x in &X {
        for y in &Y {
            Z.push(x + y);
        }
    }
	// この部分
    Z.sort();
    Z.dedup();
    return Z;
}
```

# bit全探索
```rust
/// bit全探索
fn bit_search(A: &[usize]) -> Vec<usize> {
    let mut X: Vec<usize> = vec![0; 0];

    for bit in 0..(1 << A.len()) {
        let mut sum = 0;
        for i in 0..(A.len()) {
            if bit & (1 << i) > 0 {
                sum += A[i];
            }
        }
        X.push(sum);
    }
    X.sort();
    X.dedup();
    return X;
}
```

# min関数のようなものを使いたい a16
```rust
use std::{cmp, collections::VecDeque};

cmp::min(a,b);
```

# Vecの先頭に追加したい a16
- `VecDeque`を使う
```rust
use std::collections::VecDeque;

let mut C: VecDeque<usize> = VecDeque::from(A);
```
### 参考記事
[Rustで競技プログラミングを始めた人のためのデータ構造紹介とノウハウ書き留め](https://zenn.dev/tai_calg/articles/ecbd269503dd61#%E4%BA%8C%E5%88%86%E6%8E%A2%E7%B4%A2%EF%BC%9B%E3%82%88%E3%81%8F%E4%BD%BF%E3%81%86%E3%81%AE%E3%81%AFbtreeset)
