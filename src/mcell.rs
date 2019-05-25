use std::cell::Cell;
use std::ops::Deref;
use std::ops::DerefMut;

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
        assert_unlocked();
        self.data.take()
    }

    pub(crate) fn set(&self, value: T) {
        assert_unlocked();
        self.data.set(value)
    }

    pub(crate) fn replace(&self, value: T) -> T {
        assert_unlocked();
        self.data.replace(value)
    }
}

thread_local! {
    static THREAD_LOCK: Cell<u32> = Cell::new(0);
}

const WRITE_LOCK: u32 = std::u32::MAX;

fn assert_unlocked() {
    THREAD_LOCK.with(|lock| {
        let v = lock.get();

        if v != 0 {
            panic!("cannot modify mutable data right now, lock is held");
        }
    });
}

impl<T> MCell<T> {
    /// Acquire shared access to this mcell -- but at the cost that
    /// the current thread cannot mutate **any other mcells** while
    /// the borrow is active.
    pub(crate) fn borrow(&self) -> ShareGuard<'_, T> {
        acquire_read_lock();

        // Unsafe proof obligation: we must hold the read-lock.
        unsafe { ShareGuard::new(self, self.data.as_ptr()) }
    }
}

fn acquire_read_lock() {
    THREAD_LOCK.with(|lock| {
        let v = lock.get();

        if v == WRITE_LOCK {
            panic!("cannot read from a Mut cell now");
        }

        if v == WRITE_LOCK - 1 {
            panic!("too many readers");
        }

        lock.set(v + 1);
    });
}

pub(crate) struct ShareGuard<'me, T> {
    data: &'me T,
    _thread_local: *const (),
}

impl<'me, T> ShareGuard<'me, T> {
    /// Create a new share-guard.
    ///
    /// Unsafe proof obligation:
    /// - the read lock must be held, and
    /// - `data` must come from `_cell`.
    unsafe fn new(_cell: &'me MCell<T>, data: *const T) -> Self {
        debug_assert!(THREAD_LOCK.with(|lock| lock.get() > 0));
        debug_assert_ne!(THREAD_LOCK.with(|lock| lock.get()), WRITE_LOCK);

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
        THREAD_LOCK.with(|lock| {
            let v = lock.get();
            assert!(v > 0 && v != WRITE_LOCK);
            lock.set(v - 1);
        });
    }
}

impl<T> MCell<T> {
    /// Acquire mutable access to this mcell -- but at the cost that
    /// the current thread cannot access (read or write) **any other
    /// mcells** while the borrow is active.
    pub(crate) fn borrow_mut(&self) -> MutGuard<'_, T> {
        acquire_write_lock();

        // Proof obligation: we must hold the write-lock.
        unsafe { MutGuard::new(self, self.data.as_ptr()) }
    }
}

fn acquire_write_lock() {
    THREAD_LOCK.with(|lock| {
        assert!(lock.get() == 0, "lock already held");
        lock.set(WRITE_LOCK);
    });
}

pub(crate) struct MutGuard<'me, T> {
    data: &'me mut T,
    _thread_local: *const (),
}

impl<'me, T> MutGuard<'me, T> {
    /// Create a new mut-guard.
    ///
    /// Unsafe proof obligation:
    /// - the write lock must be held, and
    /// - `data` must come from `_cell`.
    unsafe fn new(_cell: &'me MCell<T>, data: *mut T) -> Self {
        debug_assert_eq!(THREAD_LOCK.with(|lock| lock.get()), WRITE_LOCK);

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
        THREAD_LOCK.with(|lock| {
            let v = lock.get();
            assert!(v == WRITE_LOCK);
            lock.set(0);
        });
    }
}
