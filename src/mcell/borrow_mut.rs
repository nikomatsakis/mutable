use super::*;

impl<T> MCell<T> {
    /// Acquire mutable access to this mcell -- but at the cost that
    /// the current thread cannot access (read or write) **any other
    /// mcells** while the borrow is active.
    pub(crate) fn borrow_mut(&self) -> MutGuard<'_, T> {
        lock::acquire_write_lock();

        // Proof obligation: we must hold the write-lock.
        unsafe { MutGuard::new(self, self.data.as_ptr()) }
    }
}

pub(crate) struct MutGuard<'me, T> {
    data: &'me mut T,
    _thread_local: *const (),
}

impl<'me, T> MutGuard<'me, T> {
    /// Create a new mut-guard.
    ///
    /// Unsafe proof obligation:
    /// - the write lock must be held (and delegated to us), and
    /// - `data` must come from `_cell`.
    unsafe fn new(_cell: &'me MCell<T>, data: *mut T) -> Self {
        lock::debug_assert_write_locked();

        // The write lock is held so long as we exist, so will retain
        // unique access to `*data`. Moreover, we will assign it a
        // lifetime of `'me` which is tied to the cell `_cell`, so the
        // data will not be deinitialized.
        MutGuard {
            data: &mut *data,
            _thread_local: std::ptr::null(),
        }
    }
}

impl<'me, T> Deref for MutGuard<'me, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'me, T> DerefMut for MutGuard<'me, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<'me, T> Drop for MutGuard<'me, T> {
    fn drop(&mut self) {
        lock::release_write_lock();
    }
}
