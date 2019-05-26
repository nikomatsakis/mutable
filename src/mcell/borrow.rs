use super::*;

impl<T> MCell<T> {
    /// Acquire shared access to this mcell -- but at the cost that
    /// the current thread cannot mutate **any other mcells** while
    /// the borrow is active.
    pub(crate) fn borrow(&self) -> ShareGuard<'_, T> {
        lock::acquire_read_lock();

        // Unsafe proof obligation: we must hold the read-lock.
        unsafe { ShareGuard::new(self, self.data.as_ptr()) }
    }
}

pub(crate) struct ShareGuard<'me, T> {
    data: &'me T,
    _thread_local: *const (),
}

impl<'me, T> ShareGuard<'me, T> {
    /// Create a new share-guard.
    ///
    /// Unsafe proof obligation:
    /// - the read lock must be held (and delegated to us), and
    /// - `data` must come from `_cell`.
    unsafe fn new(_cell: &'me MCell<T>, data: *const T) -> Self {
        lock::debug_assert_read_locked();

        // The write lock is held so long as we exist, so will retain
        // unique access to `*data`. Moreover, we will assign it a
        // lifetime of `'me` which is tied to the cell `_cell`, so the
        // data will not be deinitialized.
        ShareGuard {
            data: &*data,
            _thread_local: std::ptr::null(),
        }
    }
}

impl<'me, T> Deref for ShareGuard<'me, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'me, T> Drop for ShareGuard<'me, T> {
    fn drop(&mut self) {
        lock::release_read_lock();
    }
}
