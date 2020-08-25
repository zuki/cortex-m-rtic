//! examples/preempt.rs

#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;
use rtic::app;

// [訳注] stm32f307にはInterrupt::GPIA-CはないのでSPI1-3に変更
#[app(device = stm32f4::stm32f407)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::SPI1);
    }

    #[task(binds = SPI1, priority = 1)]
    fn spi1(_: spi1::Context) {
        hprintln!("SPI1 - start").unwrap();
        rtic::pend(Interrupt::SPI3);
        hprintln!("SPI1 - end").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(binds = SPI2, priority = 2)]
    fn spi2(_: spi2::Context) {
        hprintln!(" SPI2").unwrap();
    }

    #[task(binds = SPI3, priority = 2)]
    fn spi3(_: spi3::Context) {
        hprintln!(" SPI3 - start").unwrap();
        rtic::pend(Interrupt::SPI2);
        hprintln!(" SPI3 - end").unwrap();
    }
};
