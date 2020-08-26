# ヒントとコツ

## ジェネリクス

リソースは、タスクの優先度に応じて、リソースプロキシまたはユニーク参照（`&mut-`）としてコンテキストに現れることがあります。同じリソースが異なるコンテキストで*異なる*型として現れることがあるため、リソースを使用する共通な操作をプレーンな関数にリファクタリングすることはできません。しかし、*ジェネリック*を使用することでそのようなリファクタリングは可能です。

すべてのリソースプロキシは`rtic::Mutex`トレイトを実装しています。一方、ユニーク参照（`&mut-`）は（トレイトシステムの制限のため）このトレイトを実装していません。しかし、これらの参照を`Mutex`トレイトを実装しているニュータイプ[`rtic::Exclusive`]でラップすることができます。このニュータイプの助けを借りることで、リソース全般を操作する汎用関数を書くことができ、これを異なるタスクから呼び出して、同一のリソースセットに対して何らかの操作を実行することができます。以下にそのような例を示します。

[`rtic::Exclusive`]: ../../../api/rtic/struct.Exclusive.html

``` rust
{{#include ../../../../examples/generics.rs}}
```

``` console
$ cargo run --example generics
{{#include ../../../../ci/expected/generics.run}}
```

また、ジェネリックを使用することで、都度大量のコードを書き換えることなく開発中のタスクの静的な優先順位を変更することができます。


## 条件付きコンパイル

リソース（`struct Resources`のフィールド）とタスク（`fn`アイテム）に対して条件付きコンパイル（`#[cfg]`）を使用することができます。`[cfg] `属性を使用する効果は、条件が満たされないと対応する`Context`  `struct`からリソースやタスクが利用できなくなることです。

以下の例では、`foo`タスクがスポーンされるたびにメッセージを出力しますが、それはプログラムが`dev`プロファイルを使ってコンパイルされた場合に限ります。

``` rust
{{#include ../../../../examples/cfg.rs}}
```

``` console
$ cargo run --example cfg --release

$ cargo run --example cfg
{{#include ../../../../ci/expected/cfg.run}}
```

## RAMからタスクを実行する

RTIC v0.4.0でRTICアプリケーションの仕様を属性に移行した主な目的は、他の属性との相互運用を可能にすることでした。たとえば、`link_section`属性はタスクに適用することでタスクをRAMに配置することができます。

> 重要: 一般的に、`link_section`、`export_name`、`no_mangle`の各属性は非常に
> 強力ですが、誤用しやすい属性でもあります。これらの属性を誤用すると、未定義動作を
> 引き起こす可能性があります。これらの属性の使用は避け、`cortex-m-rt` の
> `interrupt`や`exception`属性のような、安全な上位レベルの属性を常に使用するように
> してください。
>
> RAM関数のような特別な場合には、`Cortex-m-rt` v0.6.5にそれに合った安全な抽象化は
> ありませんが、将来のリリースで`ramfunc`属性を追加するための[RFC]があります。

[RFC]: https://github.com/rust-embedded/cortex-m-rt/pull/100

以下の例は、優先度の高いタスク`bar`をRAMに配置する方法を示しています。

``` rust
{{#include ../../../../examples/ramfunc.rs}}
```

このプログラムを実行すると期待した出力が得られる。

``` console
$ cargo run --example ramfunc
{{#include ../../../../ci/expected/ramfunc.run}}
```

One can look at the output of `cargo-nm`の出力を見ると、`bar`がRAM
（`0x2000_0000`）に、`foo`がFlash（`0x0000_0000`）にあることが確認できます。

``` console
$ cargo nm --example ramfunc --release | grep ' ramfunc::foo::'
{{#include ../../../../ci/expected/ramfunc.grep.foo}}
```

``` console
$ cargo nm --example ramfunc --release | grep ' ramfunc::bar::'
{{#include ../../../../ci/expected/ramfunc.grep.bar}}
```

## より高速なメッセージ渡しのための間接渡し

メッセージの受け渡しでは、常にペイロードのコピーが、送信側からスタティック変数へ、
さらに、スタティック変数から受信側へと行われます。したがって、`[u8; 128]`のような
大きなバッファをメッセージとして送信すると高価な`memocpy`が2回行われることになります。
メッセージ渡しのオーバーヘッドを最小限にするには、値でバッファを送信する代わりに、
バッファの所有ポインタを送信する間接渡しを使用することができます。

間接渡しを実現するためには（`alloc::Box`や`alloc::Rc`などの）グローバル
アロケータ(alloc::Box, alloc::Rcなど)を使用することができますが、これには
Rust v1.37.0のnightlyチャネルの使用が必要です。そうでなければ、[`heapless::Pool`]
のような静的にアロケートするメモリプールを使用することができます。

[`heapless::Pool`]: https://docs.rs/heapless/0.5.0/heapless/pool/index.html

ここでは、`heapless::Pool`を使用して 128バイトの「ボックス」バッファを使用する例を示します。

``` rust
{{#include ../../../../examples/pool.rs}}
```
``` console
$ cargo run --example pool
{{#include ../../../../ci/expected/pool.run}}
```

## 展開されたコードを調べる

`#[rtic::app]`は、サポートコードを生成する手続き型マクロです。何らかの理由で
このマクロによって生成されたコードを調べる必要がある場合、2つのオプションがあります。

`target`ディレクトリにあるファイル`rtic-expansion.rs`を調べることができます。
このファイルには、（`cargo build`または`cargo check`経由で）*最後にビルドされた
*RTICアプリケーションの`#[rtic::app]`アイテム（プログラム全体ではありません！）が
展開されたコードが含まれています。展開されたコードはデフォルトではプリティプリント
されていませんので、コードを読む前には`rustfmt`を実行すると良いでしょう。

``` console
$ cargo build --example foo

$ rustfmt target/rtic-expansion.rs

$ tail target/rtic-expansion.rs
```

``` rust
#[doc = r" Implementation details"]
const APP: () = {
    #[doc = r" Always include the device crate which contains the vector table"]
    use lm3s6965 as _;
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = core::mem::transmute(());
        core.SCB.scr.modify(|r| r | 1 << 1);
        rtic::export::interrupt::enable();
        loop {
            rtic::export::wfi()
        }
    }
};
```

または、`cargo-expand`サブコマンドを使用することができます。このサブコマンドは、
`#[rtic::app]`属性を含む*すべての*マクロとクレート内のモジュールを展開し、
コンソールに出力します。

[`cargo-expand`]: https://crates.io/crates/cargo-expand

``` console
$ # 先と同じ出力を出力する
$ cargo expand --example smallest | tail
```

## リソース構造体の分割

複数のリソースを取るタスクがある場合、リソース構造体を分割することが可読性の向上に
役立ちます。これを行う方法について、2つの例を紹介します。

``` rust
{{#include ../../../../examples/destructure.rs}}
```
