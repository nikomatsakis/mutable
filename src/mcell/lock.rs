//! The thread-lock lock used by mcell in its borrow/check-out operations.

use std::cell::Cell;

thread_local! {
    static THREAD_LOCK: Cell<u32> = Cell::new(0);
}

const WRITE_LOCK: u32 = std::u32::MAX;

pub(super) fn assert_unlocked() {
    THREAD_LOCK.with(|lock| {
        let v = lock.get();

        if v != 0 {
            panic!("cannot modify mutable data right now, lock is held");
        }
    });
}

pub(super) fn debug_assert_read_locked() {
    debug_assert!(THREAD_LOCK.with(|lock| lock.get() > 0));
    debug_assert_ne!(THREAD_LOCK.with(|lock| lock.get()), WRITE_LOCK);
}

pub(super) fn debug_assert_write_locked() {
    debug_assert_eq!(THREAD_LOCK.with(|lock| lock.get()), WRITE_LOCK);
}

pub(super) fn acquire_read_lock() {
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

pub(super) fn release_read_lock() {
    THREAD_LOCK.with(|lock| {
        let v = lock.get();
        assert!(v > 0 && v != WRITE_LOCK);
        lock.set(v - 1);
    });
}

pub(super) fn acquire_write_lock() {
    THREAD_LOCK.with(|lock| {
        assert!(lock.get() == 0, "lock already held");
        lock.set(WRITE_LOCK);
    });
}

pub(super) fn release_write_lock() {
    THREAD_LOCK.with(|lock| {
        let v = lock.get();
        assert!(v == WRITE_LOCK);
        lock.set(0);
    });
}
