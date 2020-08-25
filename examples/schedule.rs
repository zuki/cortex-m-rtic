//! examples/schedule.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::peripheral::DWT;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use rtic::cyccnt::{Instant, U32Ext as _};

// NOTE: does NOT work on QEMU!
#[rtic::app(device = stm32f4::stm32f407, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init(schedule = [foo, bar])]
    fn init(mut cx: init::Context) {
        // モノトニックタイマー(CCYCNT)を初期化（有効化）
        cx.core.DCB.enable_trace();
        // ソフトウェアがDWTをロックするCortex-M7デバイスで必要（STM32F7など）
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();

        // 意味的には、モノトニックタイマーは `init` の間、時間 "0" でフリーズします。
        // 注意: このコンテキストでは`Instant::now`をコールしては*いけません*。
        // もしした場合は、意味のない値が返ります。
        let now = cx.start; // システムの開始時間

        hprintln!("init @ {:?}", now).unwrap();

        // `foo`を8e6サイクル（クロックサイクル）後に実行するようスケジュール
        cx.schedule.foo(now + 8_000_000.cycles()).unwrap();

        // `bar`を4e6サイクル（クロックサイクル）後に実行するようスケジュール
        cx.schedule.bar(now + 4_000_000.cycles()).unwrap();
    }

    #[task]
    fn foo(_: foo::Context) {
        hprintln!("foo  @ {:?}", Instant::now()).unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar  @ {:?}", Instant::now()).unwrap();
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
    }
};
