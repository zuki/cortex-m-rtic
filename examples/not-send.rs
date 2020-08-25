//! `examples/not-send.rs`

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use core::marker::PhantomData;

use cortex_m_semihosting::debug;
use panic_halt as _;
use rtic::app;

pub struct NotSend {
    _0: PhantomData<*const ()>,
}

#[app(device = stm32f4::stm32f407)]
const APP: () = {
    struct Resources {
        #[init(None)]
        shared: Option<NotSend>,
    }

    #[init(spawn = [baz, quux])]
    fn init(c: init::Context) {
        c.spawn.baz().unwrap();
        c.spawn.quux().unwrap();
    }

    #[task(spawn = [bar])]
    fn foo(c: foo::Context) {
        // シナリオ 1: 同じ優先度で実行しているタスクに渡されるメッセージ
        c.spawn.bar(NotSend { _0: PhantomData }).ok();
    }

    #[task]
    fn bar(_: bar::Context, _x: NotSend) {
        // シナリオ 1
    }

    #[task(priority = 2, resources = [shared])]
    fn baz(c: baz::Context) {
        // シナリオ 2: 同じ優先度で実行しているタスク間で共有されるリソース
        *c.resources.shared = Some(NotSend { _0: PhantomData });
    }

    #[task(priority = 2, resources = [shared])]
    fn quux(c: quux::Context) {
        // シナリオ 2
        let _not_send = c.resources.shared.take().unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }

    // RTICはソフトウェアタスクを使用する際、未使用の割り込みをexternブロックで
    // 宣言する必要がある。これらの未使用の割り込みはソフトウェアタスクのディスパッチに
    // 使用される。
    extern "C" {
        fn ETH();
        fn CRYP();
    }
};
