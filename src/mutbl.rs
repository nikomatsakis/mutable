use crate::mcell::MCell;

pub struct Mut<T> {
    data: MCell<T>,
}

impl<T> Mut<T> {
    pub fn new(value: T) -> Self {
        Mut {
            data: MCell::new(value),
        }
    }

    pub fn replace(&self, new_value: T) -> T {
        self.data.replace(new_value)
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.data.borrow().clone()
    }

    pub fn set(&self, new_value: T) {
        self.data.set(new_value)
    }
}
