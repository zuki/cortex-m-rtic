//! examples/smallest.rs

#![no_main]
#![no_std]

use stm32f4::stm32f407;

use panic_semihosting as _; // panic handler
use rtic::app;

#[app(device = stm32f407)]
const APP: () = {};
