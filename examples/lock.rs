//! examples/lock.rs

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
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) {
        rtic::pend(Interrupt::SPI1);
    }

    // 優先度の指定がない場合は`1`が仮定される
    #[task(binds = SPI1, resources = [shared])]
    fn spi1(mut c: spi1::Context) {
        hprintln!("A").unwrap();

        // 優先度の低いタスクはデータのアクセスにクリティカルセクションを必要とする
        c.resources.shared.lock(|shared| {
            // データはこのクリティカルセクション内においてのみ変更できる（クロージャ）
            *shared += 1;

            // クリティカルセクション内であるのでSPI2は直ちに実行*できない*
            rtic::pend(Interrupt::SPI2);

            hprintln!("B - shared = {}", *shared).unwrap();

            // SPI3は`shared`を争っていないので、今すぐ実行できる
            rtic::pend(Interrupt::SPI3);
        });

        // クリティカルセクションを抜けた。ここでSPI2は開始できる

        hprintln!("E").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(binds = SPI2, priority = 2, resources = [shared])]
    fn spi2(c: spi2::Context) {
        // 高い優先度を持つタスクはクリティカルセクションを必要と*しない*
        *c.resources.shared += 1;

        hprintln!("D - shared = {}", *c.resources.shared).unwrap();
    }

    #[task(binds = SPI3, priority = 3)]
    fn spi3(_: spi3::Context) {
        hprintln!("C").unwrap();
    }
};
