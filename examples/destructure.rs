//! examples/destructure.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    struct Resources {
        // 使用するいくつかのリソース
        #[init(0)]
        a: u32,
        #[init(0)]
        b: u32,
        #[init(0)]
        c: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::SPI1);
        rtic::pend(Interrupt::SPI2);
    }

    // 直接構造体を分割
    #[task(binds = SPI1, resources = [a, b, c])]
    fn spi1(cx: spi1::Context) {
        let a = cx.resources.a;
        let b = cx.resources.b;
        let c = cx.resources.c;

        hprintln!("SPI1: a = {}, b = {}, c = {}", a, b, c).unwrap();
    }

    // 構造体分割構文
    #[task(binds = SPI2, resources = [a, b, c])]
    fn spi2(cx: spi2::Context) {
        let spi2::Resources { a, b, c } = cx.resources;

        hprintln!("SPI2: a = {}, b = {}, c = {}", a, b, c).unwrap();
    }
};
