//! examples/generics.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;
use rtic::{Exclusive, Mutex};

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    struct Resources {
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::SPI1);
        rtic::pend(Interrupt::SPI2);
    }

    #[task(binds = SPI1, resources = [shared])]
    fn spi1(c: spi1::Context) {
        static mut STATE: u32 = 0;

        hprintln!("SPI1(STATE = {})", *STATE).unwrap();

        // 第２引数は型`resources::shared`を持つ
        advance(STATE, c.resources.shared);

        rtic::pend(Interrupt::SPI2);

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(binds = SPI2, priority = 2, resources = [shared])]
    fn spi2(c: spi2::Context) {
        static mut STATE: u32 = 0;

        hprintln!("SPI2(STATE = {})", *STATE).unwrap();

        // `shared`に直接アクセスできることを示すだけ
        *c.resources.shared += 0;

        // 第2引数は型`Exclusive<u32>`を持つ
        advance(STATE, Exclusive(c.resources.shared));
    }
};

// 第2パラメタはジェネリクス: `Mutex`トレイトを実装している任意のタイプを受け付ける
fn advance(state: &mut u32, mut shared: impl Mutex<T = u32>) {
    *state += 1;

    let (old, new) = shared.lock(|shared: &mut u32| {
        let old = *shared;
        *shared += *state;
        (old, *shared)
    });

    hprintln!("shared: {} -> {}", old, new).unwrap();
}
