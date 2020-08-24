## リソース

このフレームワークは、前章で見たコンテキスト(タスクハンドラ、`init`、`idle`)間でデータを共有するための抽象化であるリソースを提供します。

リソースは`#[app]`疑似モジュール内で宣言された関数にだけ見えるデータです。フレームワークは、どのコンテキストがどのリソースにアクセスできるかをユーザが完全にコントロールできるようにします。

すべてのリソースは`#[app]`擬似モジュールにおいて1つの`struct`として宣言されます。構造体の各フィールドは様々なリソースに対応します。リソースには`#[init]`属性を使用して初期値を与えることができます。初期値が与えられないリソースは*遅延*リソースと呼ばれ、このページの後続セクションで詳しく説明します。

各コンテキスト(タスクハンドラ、`init`、`idle`)は、アクセスしようとするリソースを対応するメタデータ属性の`resources`引数を使って宣言しなければなりません。この引数は、リソース名のリストを値として受け取ります。リストされたリソースは`Context`構造体の`resources`フィールドとしてコンテキストが利用できるようになります。

以下の例は、`shared`という名前のリソースへのアクセスを2つの割り込みハンドラが共有していることを示しています。

``` rust
{{#include ../../../../examples/resource.rs}}
```

``` console
$ cargo run --example resource
{{#include ../../../../ci/expected/resource.run}}
```

`idle`からは`shared`リソースにアクセスできないことに注意してください。これを試みるとコンパイルエラーになります。

## `lock`

プリエンプションがある場合、データ競合が発生しないように共有データを変更するにはクリティカルセクションが必要になります。フレームワークはタスクの優先順位とどのタスクがどのリソースにアクセスできるかを完全に把握しているので、メモリの安全性のために必要な場合クリティカルセクションの使用を強制します。

クリティカルセクションが必要な場合、フレームワークは参照ではなくリソースプロキシを渡します。このリソースプロキシは[`Mutex`]トレイトを実装した構造体です。このトレイトの唯一のメソッドである[`lock`]は、そのクロージャ引数をクリティカルセクションで実行します。

[`Mutex`]: ../../../api/rtic/trait.Mutex.html
[`lock`]: ../../../api/rtic/trait.Mutex.html#method.lock

`lock` APIで作成されるクリティカルセクションは動的優先度に基づいています。すなわち、コンテキストの動的優先度を一時的に*上限*優先度に引き上げ、他のタスクがクリティカルセクションをプリエンプションしないようにします。この同期プロトコルは、[Immediate Ceiling Priority Protocol (ICPP)][icpp]として知られています。

[icpp]: https://en.wikipedia.org/wiki/Priority_ceiling_protocol

以下の例では、優先度が1から3までの3つの割り込みハンドラを使用しています。優先度の低い2つのハンドラが`shared`リソースを争っています。最低の優先度を持つハンドラはデータのアクセスに`shared`リソースの`lock`が必要ですが、中間の優先度を持つタスクは直接データにアクセスすることができます。最高の優先度を持つハンドラは（`shared`リソースにはアクセスできませんが）最低の優先度を持つハンドラによって作成されたクリティカルセクションを自由にプリエンプションできます。

``` rust
{{#include ../../../../examples/lock.rs}}
```

``` console
$ cargo run --example lock
{{#include ../../../../ci/expected/lock.run}}
```

## 遅延リソース

遅延リソースとは、`#[init]`属性を使ってコンパイル時に初期値が与えられず、代わりに`init`関数が返す`init::LateResources`の値を使って実行時に初期化されるリソースのことです。

遅延リソースは、`init`で初期化されたペリフェラルを（所有権の移行という形で）*移動*するのに便利です。

以下の例では、`UART0`割り込みハンドラと`idle`タスクの間にロックのない一方向のチャネルを確立するのにレイトリソースを使用しています。一つのプロデューサと一つのコンシューマからなる[`Queue`]が、チャネルとして使用されています。キューは`init`においてコンシューマとプロデューサの２つのエンドポイントに分割され、各エンドポイントが異なるリソースに格納されています。すなわち、`UART0`がプロデューサリソースを、`idle`がコンシューマリソースを所有しています。

[`Queue`]: ../../../api/heapless/spsc/struct.Queue.html


``` rust
{{#include ../../../../examples/late.rs}}
```

``` console
$ cargo run --example late
{{#include ../../../../ci/expected/late.run}}
```

## 共有アクセスのみ

フレームワークは、デフォルトでは、すべてのタスクがリソースへの排他的アクセス(`&mut-`)を必要であると仮定しますが、`resources`リストに`&resource_name`構文を使うことでタスクがリソースへの共有アクセス(`&-`)のみを必要とすることを指定することができます。

リソースへの共有アクセス(`&-`)を指定する利点は、異なる優先度で実行されている複数のタスクでリソースが競合するような場合でも、リソースにアクセスする際にロックが必要ないことです。マイナス面は、タスクがリソースへの共有参照(`&-`)を取得するだけなのでリソースに対して実行できる操作が制限されることです。ただし、共有参照で十分な場合、このアプローチは必要なロックの数を減らします。

このリリースのRTICでは、*ある*リソースに対して異なるタスクから排他的アクセス(`&mut-`)と共有アクセス(`&-`)の両方を要求することはできないことに注意してください。そうしようとするとコンパイルエラーになります。

以下の例では、鍵(例えば暗号鍵)が実行時にロード(または作成)され、異なる優先度で実行する2つのタスクからロックなしで使用されています。

``` rust
{{#include ../../../../examples/only-shared-access.rs}}
```

``` console
$ cargo run --example only-shared-access
{{#include ../../../../ci/expected/only-shared-access.run}}
```