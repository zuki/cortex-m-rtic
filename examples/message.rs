//! examples/message.rs

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
        c.spawn.foo(/* メッセージなし */).unwrap();
    }

    #[task(spawn = [bar])]
    fn foo(c: foo::Context) {
        static mut COUNT: u32 = 0;

        hprintln!("foo").unwrap();

        c.spawn.bar(*COUNT).unwrap();
        *COUNT += 1;
    }

    #[task(spawn = [baz])]
    fn bar(c: bar::Context, x: u32) {
        hprintln!("bar({})", x).unwrap();

        c.spawn.baz(x + 1, x + 2).unwrap();
    }

    #[task(spawn = [foo])]
    fn baz(c: baz::Context, x: u32, y: u32) {
        hprintln!("baz({}, {})", x, y).unwrap();

        if x + y > 4 {
            debug::exit(debug::EXIT_SUCCESS);
            loop {}     // 実機ではdebug::exit()では止まらない
        }

        c.spawn.foo().unwrap();
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
    }
};
