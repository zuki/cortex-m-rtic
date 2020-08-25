//! examples/types.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::debug;
use panic_semihosting as _;
use rtic::cyccnt;

#[rtic::app(device = stm32f4::stm32f407, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        #[init(0)]
        shared: u32,
    }

    #[init(schedule = [foo], spawn = [foo])]
    fn init(cx: init::Context) {
        let _: cyccnt::Instant = cx.start;
        let _: rtic::Peripherals = cx.core;
        let _: stm32f4::stm32f407::Peripherals = cx.device;
        let _: init::Schedule = cx.schedule;
        let _: init::Spawn = cx.spawn;

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[idle(schedule = [foo], spawn = [foo])]
    fn idle(cx: idle::Context) -> ! {
        let _: idle::Schedule = cx.schedule;
        let _: idle::Spawn = cx.spawn;

        loop {}
    }

    #[task(binds = USART1, resources = [shared], schedule = [foo], spawn = [foo])]
    fn usart1(cx: usart1::Context) {
        let _: cyccnt::Instant = cx.start;
        let _: resources::shared = cx.resources.shared;
        let _: usart1::Schedule = cx.schedule;
        let _: usart1::Spawn = cx.spawn;
    }

    #[task(priority = 2, resources = [shared], schedule = [foo], spawn = [foo])]
    fn foo(cx: foo::Context) {
        let _: cyccnt::Instant = cx.scheduled;
        let _: &mut u32 = cx.resources.shared;
        let _: foo::Resources = cx.resources;
        let _: foo::Schedule = cx.schedule;
        let _: foo::Spawn = cx.spawn;
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
    }
};
