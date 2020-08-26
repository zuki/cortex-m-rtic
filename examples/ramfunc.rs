//! examples/ramfunc.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    #[init(spawn = [bar])]
    fn init(c: init::Context) {
        c.spawn.bar().unwrap();
    }

    #[inline(never)]
    #[task]
    fn foo(_: foo::Context) {
        hprintln!("foo").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    // このタスクをRAMから実行
    #[inline(never)]
    #[link_section = ".data.bar"]
    #[task(priority = 2, spawn = [foo])]
    fn bar(c: bar::Context) {
        c.spawn.foo().unwrap();
    }

    extern "C" {
        fn SPI1();

        // RAMからタスクディスパッチャを実行
        #[link_section = ".data.SPI2"]
        fn SPI2();
    }
};
