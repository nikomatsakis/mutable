use crate::pure_clone::PureClone;
use std::cell::Cell;

pub struct Mut<T> {
    data: Cell<T>,
}

impl<T> Mut<T> {
    pub fn new(value: T) {
        Mut { data: Cell::new(value) }
    }

    pub fn replace(&self, new_value: T) -> T {
        self.data.swap(new_value)
    }

    pub fn get(&self) -> T
    where
        T: PureClone,
    {
        let ptr: *mut T = self.data.as_ptr();

        // Unsafe proof obligation: We need to know that nobody will
        // mutate the data in `self.data`. Altough we are invoking a
        // user-supplied clone, pure-clone gives us this obligation.
        //
        // TODO -- express this more cleanly =)
        let ptr: &T = unsafe { &*ptr };
        ptr.clone()
    }

    pub fn set(&self, new_value: T) -> T {
        self.data.set(new_value)
    }

    pub fn swap(&self, new_value: &Mut<T>) -> T {
        self.data.swap(&new_value.data)
    }
}
