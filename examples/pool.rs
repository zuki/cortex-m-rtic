//! examples/pool.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use heapless::{
    pool,
    pool::singleton::{Box, Pool},
};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;
use rtic::app;

// 128バイトのメモリブロックのプールを宣言する
pool!(P: [u8; 128]);

#[app(device = stm32f4::stm32f407)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        static mut MEMORY: [u8; 512] = [0; 512];

        // メモリプールの容量を4倍に増加させる
        P::grow(MEMORY);

        rtic::pend(Interrupt::SPI1);
    }

    #[task(binds = SPI1, priority = 2, spawn = [foo, bar])]
    fn spi1(c: spi1::Context) {
        // メモリブロックを要求し、初期化はしない
        let x = P::alloc().unwrap().freeze();

        // タスク`foo`にブロックを送信
        c.spawn.foo(x).ok().unwrap();

        // タスク`bar`に別のブロックを送信
        c.spawn.bar(P::alloc().unwrap().freeze()).ok().unwrap();
    }

    #[task]
    fn foo(_: foo::Context, x: Box<P>) {
        hprintln!("foo({:?})", x.as_ptr()).unwrap();

        // 明示的にブロックをプールに返す
        drop(x);

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(priority = 2)]
    fn bar(_: bar::Context, x: Box<P>) {
        hprintln!("bar({:?})", x.as_ptr()).unwrap();

        // これは自動的に行われるため、`drop`の呼び出しは削除できる
        // drop(x);
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
        fn CRYP();
    }
};
