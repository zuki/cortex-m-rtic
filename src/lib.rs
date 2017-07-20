#![feature(asm)]
#![feature(const_fn)]
#![feature(optin_builtin_traits)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rtfm_macros;
extern crate static_ref;

use core::cell::UnsafeCell;

pub use cortex_m_rtfm_macros::app;
pub use cortex_m::asm::{bkpt, wfi};
pub use cortex_m::interrupt::{self, CriticalSection};
pub use cortex_m::interrupt::free as atomic;
#[doc(hidden)]
pub use cortex_m::register::basepri as _basepri;
pub use static_ref::Static;
use cortex_m::interrupt::Nr;
#[cfg(not(armv6m))]
use cortex_m::register::basepri;
use cortex_m::register::primask;

#[cfg(not(armv6m))]
macro_rules! barrier {
    () => {
        asm!("" ::: "memory" : "volatile");
    }
}

#[inline(always)]
unsafe fn claim<T, U, R, F, G>(
    data: T,
    ceiling: u8,
    nvic_prio_bits: u8,
    f: F,
    g: G,
) -> R
where
    F: FnOnce(U) -> R,
    G: FnOnce(T) -> U,
{
    let max_priority = 1 << nvic_prio_bits;

    if primask::read().is_active() {
        // Interrupts disabled. We are in a *global* critical section; we can
        // directly access the data
        f(g(data))
    } else {
        match () {
            #[cfg(armv6)]
            () => {
                interrupt::disable();
                let ret = f(g(data));
                interrupt::enable();
                ret
            }
            #[cfg(not(armv6))]
            () => {
                if ceiling == max_priority {
                    // Can't raise the preemption threshold to match this
                    // ceiling value. Use a *global* critical section
                    interrupt::disable();
                    let ret = f(g(data));
                    interrupt::enable();
                    ret
                } else {
                    // current preemption threshold (hardware value)
                    let old = basepri::read();
                    // logical value of ^
                    let t = (max_priority - old) >> (8 - nvic_prio_bits);

                    if ceiling > t {
                        // Raise the preemption threshold to protect the data
                        let hw = (max_priority - ceiling) <<
                            (8 - nvic_prio_bits);
                        basepri::write(hw);
                        barrier!();
                        let ret = f(g(data));
                        barrier!();
                        basepri::write(old);
                        ret
                    } else {
                        // The preemption threshold is high enough. Access to
                        // the data is data race free
                        f(g(data))
                    }
                }
            }
        }
    }
}

pub struct Peripheral<P>
where
    P: 'static,
{
    // FIXME(rustc/LLVM bug?) storing the ceiling in the resource de-optimizes
    // claims (the ceiling value gets loaded at runtime rather than inlined)
    // ceiling: u8,
    peripheral: cortex_m::peripheral::Peripheral<P>,
}

impl<P> Peripheral<P> {
    pub const fn new(peripheral: cortex_m::peripheral::Peripheral<P>) -> Self {
        Peripheral { peripheral }
    }

    #[inline(always)]
    pub unsafe fn borrow<'cs>(
        &'static self,
        _cs: &'cs CriticalSection,
    ) -> &'cs P {
        &*self.peripheral.get()
    }

    #[inline(always)]
    pub unsafe fn claim<R, F>(
        &'static self,
        ceiling: u8,
        nvic_prio_bits: u8,
        f: F,
    ) -> R
    where
        F: FnOnce(&P) -> R,
    {
        claim(&self.peripheral, ceiling, nvic_prio_bits, f, |peripheral| {
            &*peripheral.get()
        })
    }

    pub fn get(&self) -> *mut P {
        self.peripheral.get()
    }
}

unsafe impl<P> Sync for Peripheral<P>
where
    P: Send,
{
}

pub struct Resource<T> {
    // FIXME(rustc/LLVM bug?) storing the ceiling in the resource de-optimizes
    // claims (the ceiling value gets loaded at runtime rather than inlined)
    // ceiling: u8,
    data: UnsafeCell<T>,
}

impl<T> Resource<T> {
    pub const fn new(value: T) -> Self {
        Resource {
            data: UnsafeCell::new(value),
        }
    }

    #[inline(always)]
    pub unsafe fn borrow<'cs>(
        &'static self,
        _cs: &'cs CriticalSection,
    ) -> &'cs Static<T> {
        Static::ref_(&*self.data.get())
    }

    #[inline(always)]
    pub unsafe fn borrow_mut<'cs>(
        &'static self,
        _cs: &'cs CriticalSection,
    ) -> &'cs mut Static<T> {
        Static::ref_mut(&mut *self.data.get())
    }

    #[inline(always)]
    pub unsafe fn claim<R, F>(
        &'static self,
        ceiling: u8,
        nvic_prio_bits: u8,
        f: F,
    ) -> R
    where
        F: FnOnce(&Static<T>) -> R,
    {
        claim(&self.data, ceiling, nvic_prio_bits, f, |data| {
            Static::ref_(&*data.get())
        })
    }

    #[inline(always)]
    pub unsafe fn claim_mut<R, F>(
        &'static self,
        ceiling: u8,
        nvic_prio_bits: u8,
        f: F,
    ) -> R
    where
        F: FnOnce(&mut Static<T>) -> R,
    {
        claim(&self.data, ceiling, nvic_prio_bits, f, |data| {
            Static::ref_mut(&mut *data.get())
        })
    }

    pub fn get(&self) -> *mut T {
        self.data.get()
    }
}

unsafe impl<T> Sync for Resource<T>
where
    T: Send,
{
}

/// Sets an interrupt as pending
pub fn set_pending<I>(interrupt: I)
where
    I: Nr,
{
    // NOTE(safe) atomic write
    let nvic = unsafe { &*cortex_m::peripheral::NVIC.get() };
    nvic.set_pending(interrupt);
}

#[macro_export]
macro_rules! task {
    ($device:ident, $NAME:ident, $body:path) => {
        #[allow(non_snake_case)]
        #[allow(unsafe_code)]
        #[no_mangle]
        pub unsafe extern "C" fn $NAME() {
            let f: fn(::$NAME::Resources) = $body;

            let nvic_prio_bits = $device::NVIC_PRIO_BITS;
            let max_priority = 1 << nvic_prio_bits;
            let hw = (max_priority - ::$NAME::$NAME) << (8 - nvic_prio_bits);

            let old = $crate::_basepri::read();
            $crate::_basepri::write(hw);
            f(::$NAME::Resources::new());
            $crate::_basepri::write(old);
        }
    };
    ($device:ident, $NAME:ident, $body:path, $local:ident {
        $($var:ident: $ty:ty = $expr:expr;)+
    }) => {
        struct $local {
            $($var: $ty,)+
        }

        #[allow(non_snake_case)]
        #[allow(unsafe_code)]
        #[no_mangle]
        pub unsafe extern "C" fn $NAME() {
            let f: fn(
                &mut $local,
                ::$NAME::Resources,
            ) = $body;

            static mut LOCAL: $local = $local {
                $($var: $expr,)+
            };

            let nvic_prio_bits = $device::NVIC_PRIO_BITS;
            let max_priority = 1 << nvic_prio_bits;
            let hw = (max_priority - ::$NAME::$NAME) << (8 - nvic_prio_bits);

            let old = $crate::_basepri::read();
            $crate::_basepri::write(hw);
            f(
                &mut LOCAL,
                ::$NAME::Resources::new(),
            );
            $crate::_basepri::write(old);
        }
    };
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
pub enum Exception {
    /// System service call via SWI instruction
    SVCALL,
    /// Pendable request for system service
    PENDSV,
    /// System tick timer
    SYS_TICK,
}

impl Exception {
    #[doc(hidden)]
    pub fn nr(&self) -> usize {
        match *self {
            Exception::SVCALL => 11,
            Exception::PENDSV => 14,
            Exception::SYS_TICK => 15,
        }
    }
}
