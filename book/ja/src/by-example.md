# 例によるRTIC

この章では、徐々に複雑になる例を通じてRTIC（Real-Time Interrupt-driven Concurrency）フレームワークを新規利用者に紹介します。

この章のすべての例はこのプロジェクトのGitHub [repository] にあります。また、ほとんどの例はQEMUで実行することができますので、ついていくのに特別なハードウェアは必要ありません。

[repository]: https://github.com/rtic-rs/cortex-m-rtic

お手元のパソコンで例を実行するには`qemu-system-arm`プログラムが必要になります。QEMUを含む組み込み開発環境の設定方法については [組込みRustブック] を参照ください。

[組込みRustブック]: https://rust-embedded.github.io/book/intro/install.html
