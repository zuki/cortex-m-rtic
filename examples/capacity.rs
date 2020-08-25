//! examples/capacity.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::USART1);
    }

    #[task(binds = USART1, spawn = [foo, bar])]
    fn usart1(c: usart1::Context) {
        c.spawn.foo(0).unwrap();
        c.spawn.foo(1).unwrap();
        c.spawn.foo(2).unwrap();
        c.spawn.foo(3).unwrap();

        c.spawn.bar().unwrap();
    }

    #[task(capacity = 4)]
    fn foo(_: foo::Context, x: u32) {
        hprintln!("foo({})", x).unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
    }
};
