//! examples/cfg.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::debug;
#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    struct Resources {
        #[cfg(debug_assertions)] // <- `dev`プロファイルを使った場合に`true`
        #[init(0)]
        count: u32,
    }

    #[init(spawn = [foo])]
    fn init(cx: init::Context) {
        cx.spawn.foo().unwrap();
        cx.spawn.foo().unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }

    #[task(capacity = 2, resources = [count], spawn = [log])]
    fn foo(_cx: foo::Context) {
        #[cfg(debug_assertions)]
        {
            *_cx.resources.count += 1;

            _cx.spawn.log(*_cx.resources.count).unwrap();
        }

        // 次は`release`モードではコンパイルできない（コンパイルエラー）
        // *_cx.resources.count += 1;

        // ..
    }

    #[cfg(debug_assertions)]
    #[task(capacity = 2)]
    fn log(_: log::Context, n: u32) {
        hprintln!(
            "foo has been called {} time{}",
            n,
            if n == 1 { "" } else { "s" }
        )
        .ok();
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
        fn CRYP();
    }
};
