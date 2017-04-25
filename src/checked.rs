//! Run time checked `Resource`

use core::cell::RefCell;
use core::marker::PhantomData;

use cortex_m::register::{basepri, basepri_max};
use typenum::{Cmp, Less, Max, Unsigned};

use {C, GreaterThanOrEqual, LessThanOrEqual, P, U0, UMAX};

/// Run time checked `Resource`
///
/// This a convenience newtype around `Resource<RefCell<T>>`. Unlike the main
/// `Resource`, this variation can hand out a mutable reference to the inner
/// data through the `claim_mut` method.
pub struct Resource<T, C> {
    _ceiling: PhantomData<C>,
    data: RefCell<T>,
}

impl<T, RC> Resource<T, C<RC>>
where
    RC: GreaterThanOrEqual<U0>,
    RC: LessThanOrEqual<UMAX>,
{
    /// Creates a new resource
    pub const fn new(data: T) -> Self {
        Resource {
            _ceiling: PhantomData,
            data: RefCell::new(data),
        }
    }

    /// Claims the resource
    ///
    /// # Panics
    ///
    /// This method inherits `RefCell.borrow` semantics.
    pub fn claim<R, TP, SC, F>(
        &'static self,
        _priority: &P<TP>,
        _system_ceiling: &C<SC>,
        f: F,
    ) -> R
    where
        F: FnOnce(&T, &C<<SC as Max<RC>>::Output>) -> R,
        // NOTE(Cmp) Can't claim a C16 resource
        RC: Cmp<UMAX, Output = Less> + Unsigned,
        SC: Max<RC> + Unsigned,
    {
        if RC::to_u8() > SC::to_u8() {
            unsafe {
                let old_basepri = basepri::read();
                basepri_max::write(::logical2hw(RC::to_u8()));
                barrier!();
                let ret = f(&*self.data.borrow(), &C { _marker: PhantomData });
                barrier!();
                basepri::write(old_basepri);
                ret
            }
        } else {
            f(&*self.data.borrow(), &C { _marker: PhantomData })
        }
    }

    /// Mutably claims the resource
    ///
    /// # Panics
    ///
    /// This method inherits `RefCell.borrow_mut` semantics.
    pub fn claim_mut<R, TP, SC, F>(
        &'static self,
        _priority: &P<TP>,
        _system_ceiling: &C<SC>,
        f: F,
    ) -> R
    where
        F: FnOnce(&mut T, &C<<SC as Max<RC>>::Output>) -> R,
        // NOTE(Cmp) Can't claim a C16 resource
        RC: Cmp<UMAX, Output = Less> + Unsigned,
        SC: Max<RC> + Unsigned,
    {
        if RC::to_u8() > SC::to_u8() {
            unsafe {
                let old_basepri = basepri::read();
                basepri_max::write(::logical2hw(RC::to_u8()));
                barrier!();
                let ret = f(
                    &mut *self.data.borrow_mut(),
                    &C { _marker: PhantomData },
                );
                barrier!();
                basepri::write(old_basepri);
                ret
            }
        } else {
            f(&mut *self.data.borrow_mut(), &C { _marker: PhantomData })
        }
    }
}

unsafe impl<T, C> Sync for Resource<T, C> {}
