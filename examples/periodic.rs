//! examples/periodic.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use rtic::cyccnt::{Instant, U32Ext};

const PERIOD: u32 = 8_000_000;

// NOTE: does NOT work on QEMU!
#[rtic::app(device = stm32f4::stm32f407, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init(schedule = [foo])]
    fn init(cx: init::Context) {
        // 削除: `CYCCNT`の初期化

        cx.schedule.foo(cx.start + PERIOD.cycles()).unwrap();
    }

    #[task(schedule = [foo])]
    fn foo(cx: foo::Context) {
        let now = Instant::now();
        hprintln!("foo(scheduled = {:?}, now = {:?})", cx.scheduled, now).unwrap();

        cx.schedule.foo(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
    }
};
