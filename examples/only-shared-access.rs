//! examples/only-shared-access.rs

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
        key: u32,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        rtic::pend(Interrupt::SPI1);
        rtic::pend(Interrupt::SPI2);

        init::LateResources { key: 0xdeadbeef }
    }

    #[task(binds = SPI1, resources = [&key])]
    fn spi1(cx: spi1::Context) {
        let key: &u32 = cx.resources.key;
        hprintln!("SPI1(key = {:#x})", key).unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(binds = SPI2, priority = 2, resources = [&key])]
    fn spi2(cx: spi2::Context) {
        hprintln!("SPI2(key = {:#x})", cx.resources.key).unwrap();
    }
};
