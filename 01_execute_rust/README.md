# Execute Rust

## Exercise

Rustのプログラムを実行する、基本的な手順を知りましょう。

## Point

* Rustはコンパイルが必要な言語です。実行する前に、`rustc`でコンパイルします。なお、拡張子は`.rs`になります。
  * `rustc ex01.rs`
  * コンパイルが終了したら、作成されたバイナリファイルを実行します
  * `./ex01`
* 各ソースコードの文末には`;`が必要です。忘れないようにしましょう。
* 変数の宣言は`let`を通じて行います。
  * `let integer = 1;`
  * Rustでは型推論が行われるため型を指定する必要はありませんが、[明示する場合は後置で指定します](https://rustbyexample.com/primitives.html)。
  * `let integer: i8 = 1;`
* Rustにおいては、変数は[デフォルトでは変更不可能です(immutable)](https://rustbyexample.com/variable_bindings/mut.html)。変更可能にしたい場合は、`mut`をつけます。
  * `let mut message = "Hellow World";`
* [Rustの文字列](https://rustbyexample.com/std/str.html)は、`&str`と`String`の二種類があります。
  * デフォルトでは`&str`で、`&str`は文字配列への参照という扱いになります(string sliceと呼ばれます)。
  * 長さは固定です
  * 参照のみにしたい場合は、`&'static str`とします。
  * `String`は、これに対し動的なメモリ割り当てが行われます(heap allocated)。そのため可変長です。
