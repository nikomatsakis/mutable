use std::cell::Cell;
use std::ops::Deref;
use std::ops::DerefMut;

mod borrow;
mod borrow_mut;
mod check_out;
mod lock;

/// Like a std cell, but supports borrow operations. The key thing is
/// that these operations simultaneously lock/unlock **all the cells
/// accessible to this thread**.  So if you do `cell.borrow()`, then
/// *all* MCell's are borrowed.
///
/// It exposes a **safe interface**.
pub(crate) struct MCell<T> {
    data: Cell<T>,
}

impl<T> MCell<T> {
    pub(crate) fn new(data: T) -> Self {
        MCell {
            data: Cell::new(data),
        }
    }

    pub(crate) fn take(&self) -> T
    where
        T: Default,
    {
        lock::assert_unlocked();
        self.data.take()
    }

    pub(crate) fn set(&self, value: T) {
        lock::assert_unlocked();
        self.data.set(value)
    }

    pub(crate) fn replace(&self, value: T) -> T {
        lock::assert_unlocked();
        self.data.replace(value)
    }
}
