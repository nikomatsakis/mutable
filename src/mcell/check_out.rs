use super::*;

impl<T: Default> MCell<T> {
    /// Gives mutable access to *just this cell*, while locking all
    /// other cells to read-only access. Any attempt to read this
    /// particular cell in that time will encounter the `T::Default`
    /// value.
    pub(crate) fn check_out(&self) -> CheckOutGuard<'_, T> {
        lock::assert_unlocked();
        lock::acquire_read_lock();
        let data = self.data.take();

        // Unsafe proof obligation: we acquired read-lock above.
        unsafe { CheckOutGuard::new(self, data) }
    }

    /// Gives mutable access to *just this cell*, while locking all
    /// other cells to read-only access. Any attempt to read this
    /// particular cell in that time will encounter the `T::Default`
    /// value. **This variant does not restore `self.data` on panic,
    /// but simply leaves the default value.**
    pub(crate) fn check_out_not_panic_safe<R>(&self, closure: impl FnOnce(&mut T) -> R) -> R {
        lock::assert_unlocked();
        let mut data = self.data.take();
        let _cell = self.borrow();
        let result = closure(&mut data);
        self.data.set(data);
        result
    }
}

pub(crate) struct CheckOutGuard<'me, T: Default> {
    data: T,
    cell: &'me MCell<T>,
}

impl<'me, T: Default> CheckOutGuard<'me, T> {
    /// Create a new check-out-guard.
    ///
    /// Unsafe proof obligation:
    /// - the read lock must be held (and delegated to us).
    unsafe fn new(cell: &'me MCell<T>, data: T) -> Self {
        lock::debug_assert_read_locked();

        // The write lock is held so long as we exist, so will retain
        // unique access to `*data`. Moreover, we will assign it a
        // lifetime of `'me` which is tied to the cell `_cell`, so the
        // data will not be deinitialized.
        CheckOutGuard { cell, data }
    }
}

impl<'me, T: Default> Deref for CheckOutGuard<'me, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<'me, T: Default> DerefMut for CheckOutGuard<'me, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<'me, T: Default> Drop for CheckOutGuard<'me, T> {
    fn drop(&mut self) {
        lock::release_read_lock();

        // Annoyingly, drop has an `&mut self` type that forbids us
        // from taking ownership of `self.data`, so swap the data back.
        //
        // Unsafe obligation: We are creating an `&mut` ref to the
        // interior of the cell, but we are just doing memcpy
        // operations with it and it never escapes. Further, there
        // should be no other extant `&mut` references to its interior
        // (hmm, double check that?). So should be fine.
        std::mem::swap(&mut self.data, unsafe { &mut *self.cell.data.as_ptr() })
    }
}
