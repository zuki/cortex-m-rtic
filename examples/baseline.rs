//! examples/baseline.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use stm32f4::stm32f407::Interrupt;
use panic_semihosting as _;

// 注意: QEMUでは正しく*動かない*
#[rtic::app(device = stm32f4::stm32f407, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(cx: init::Context) {
        // 削除: `CYCCNT`の初期化

        hprintln!("init(baseline = {:?})", cx.start).unwrap();

        // `foo`は`init`のベースラインである`Instant(0)`を継承
        cx.spawn.foo().unwrap();
    }

    #[task(schedule = [foo])]
    fn foo(cx: foo::Context) {
        static mut ONCE: bool = true;

        hprintln!("foo(baseline = {:?})", cx.scheduled).unwrap();

        if *ONCE {
            *ONCE = false;

            rtic::pend(Interrupt::USART1);
        } else {
            debug::exit(debug::EXIT_SUCCESS);
        }
    }

    #[task(binds = USART1, spawn = [foo])]
    fn usart1(cx: usart1::Context) {
        hprintln!("USART1(baseline = {:?})", cx.start).unwrap();

        // `foo`は`USART1`のベースラインである`start`時間を継承
        cx.spawn.foo().unwrap();
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
    }
};
