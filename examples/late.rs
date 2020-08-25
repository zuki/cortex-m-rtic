//! examples/late.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use heapless::{
    consts::*,
    i,
    spsc::{Consumer, Producer, Queue},
};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;

#[rtic::app(device = stm32f4::stm32f407)]
const APP: () = {
    // 遅延リソース
    struct Resources {
        p: Producer<'static, u32, U4>,
        c: Consumer<'static, u32, U4>,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        static mut Q: Queue<u32, U4> = Queue(i::Queue::new());

        let (p, c) = Q.split();

        // 遅延リソースの初期化
        init::LateResources { p, c }
    }

    #[idle(resources = [c])]
    fn idle(c: idle::Context) -> ! {
        loop {
            if let Some(byte) = c.resources.c.dequeue() {
                hprintln!("received message: {}", byte).unwrap();

                debug::exit(debug::EXIT_SUCCESS);
                loop {}     // 実機ではdebug::exit()では止まらない
            } else {
                rtic::pend(Interrupt::USART1);
            }
        }
    }

    #[task(binds = USART1, resources = [p])]
    fn usart1(c: usart1::Context) {
        c.resources.p.enqueue(42).unwrap();
    }
};
