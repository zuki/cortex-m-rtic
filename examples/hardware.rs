//! examples/hardware.rs

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
        // USART1割り込みを保留におく。ただし、そのハンドラは`init`の
        // リターン*後*にしか実行しない。割り込みが無効になっているからである。
        // [訳注] stm32f407にはUART0はないないのでUSART1とした。
        rtic::pend(Interrupt::USART1); // NVIC::pendに相当する

        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        // 割り込みが再度有効になる。`UART4`ハンドラはこの時点で実行する。

        hprintln!("idle").unwrap();

        rtic::pend(Interrupt::USART1);

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }

    #[task(binds = USART1)]
    fn usart1(_: usart1::Context) {
        static mut TIMES: u32 = 0;

        // ローカルの`static mut`変数に安全にアクセスできる
        *TIMES += 1;

        hprintln!(
            "USART1 called {} time{}",
            *TIMES,
            if *TIMES > 1 { "s" } else { "" }
        )
        .unwrap();
    }
};
