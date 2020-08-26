# 新たなプロジェクトを開始する

RTICフレームワークの主な機能について学びました。では、以下の手順に沿って
お手持ちのハードウェア上で試してみましょう。

1. [`cortex-m-quickstart`]テンプレートをインスタンス化します。

[`cortex-m-quickstart`]: https://github.com/rust-embedded/cortex-m-quickstart#cortex-m-quickstart

``` console
$ # たとえば、`cargo-generate`を使用します。
$ cargo generate \
    --git https://github.com/rust-embedded/cortex-m-quickstart \
    --name app

$ # 以下の手順に従がいます。
```

1. [`svd2rust`] **v0.14.x**を使って生成されたペリフェラルアクセスクレート（PAC）、 または、そのようなPACに依存するボードサポートクレートを追加します。クレートの`rt`機能を必ず有効にしてください。

[`svd2rust`]: https://crates.io/crates/svd2rust

この例では[`lm3s6965`]デバイスクレートを使用します。このデバイスクレートには`rt` Cargo featureはありません。この機能は常に有効になっています。

[`lm3s6965`]: https://crates.io/crates/lm3s6965

このデバイスクレートはターゲットデバイスのメモリレイアウトを指定したリンカースクリプトを提供しているので、`memory.x`と`build.rs`を削除する必要があります。

``` console
$ cargo add lm3s6965 --vers 0.1.3

$ rm memory.x build.rs
```

3. 依存クレートに `cortex-m-rtic` クレートを追加します。

``` console
$ cargo add cortex-m-rtic --allow-prerelease
```

4. RTICアプリケーションを書きます。

ここでは、`cortex-m-rtic`クレートの`init`サンプルを使用します。

``` console
$ curl \
    -L https://github.com/rtic-rs/cortex-m-rtic/raw/v0.5.3/examples/init.rs \
    > src/main.rs
```

この例は`panic-semihosting`クレートに依存しています。

``` console
$ cargo add panic-semihosting
```

5. プログラムをビルトし、書き込んで実行します。

``` console
$ # 注意: `.cargo/config`の`runner`オプションのコメントを外しました。
$ cargo run
{{#include ../../../../ci/expected/init.run}}```
