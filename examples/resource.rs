//! examples/resource.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    struct Resources {
        // リソース
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::SPI1);
        rtic::pend(Interrupt::SPI2);
    }

    // このコンテキストからは`shared`にアクセスできない
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        debug::exit(debug::EXIT_SUCCESS);

        // エラー: `idle::Context`には`resource`フィールドがない
        // _cx.resources.shared += 1;

        loop {}
    }

    // このコンテキストからは`shared`にアクセスできる
    #[task(binds = SPI1, resources = [shared])]
    fn spi1(cx: spi1::Context) {
        let shared: &mut u32 = cx.resources.shared;
        *shared += 1;

        hprintln!("SPI1: shared = {}", shared).unwrap();
    }

    // このコンテキストからは`shared`にアクセスできる
    #[task(binds = SPI2, resources = [shared])]
    fn spi2(cx: spi2::Context) {
        *cx.resources.shared += 1;

        hprintln!("SPI2: shared = {}", cx.resources.shared).unwrap();
    }
};
