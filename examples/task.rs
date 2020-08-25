//! examples/task.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        c.spawn.foo().unwrap();
    }

    #[task(spawn = [bar, baz])]
    fn foo(c: foo::Context) {
        hprintln!("foo - start").unwrap();

        // タスクスケジューラに`bar`を生成する
        // `foo` と `bar` は同じ優先でを持つので
        // `bar`は`foo`が終了するまで実行しない
        c.spawn.bar().unwrap();

        hprintln!("foo - middle").unwrap();

        // タスクスケジューラに`baz`を生成する
        // `baz`は`foo`より高い優先度を持つので、直ちに`foo`をプリエンプションする
        c.spawn.baz().unwrap();

        hprintln!("foo - end").unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(priority = 2)]
    fn baz(_: baz::Context) {
        hprintln!("baz").unwrap();
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
        fn CRYP();
    }
};
