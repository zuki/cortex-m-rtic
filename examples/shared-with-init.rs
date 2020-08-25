//! `examples/shared-with-init.rs`

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::debug;
use stm32f4::stm32f407::Interrupt;
use panic_halt as _;
use rtic::app;

pub struct MustBeSend;

#[app(device = stm32f4::stm32f407)]
const APP: () = {
    struct Resources {
        #[init(None)]
        shared: Option<MustBeSend>,
    }

    #[init(resources = [shared])]
    fn init(c: init::Context) {
        // この`message`は`USART1`に送られる
        let message = MustBeSend;
        *c.resources.shared = Some(message);

        rtic::pend(Interrupt::USART1);
    }

    #[task(binds = USART1, resources = [shared])]
    fn usart1(c: usart1::Context) {
        if let Some(message) = c.resources.shared.take() {
            // `message`を受け取った
            drop(message);

            debug::exit(debug::EXIT_SUCCESS);
        }
    }
};
