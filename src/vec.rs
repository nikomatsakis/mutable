use crate::mcell::MCell;
use std::vec::Vec;

mod test;

pub struct MutVec<T> {
    data: MCell<Vec<T>>,
}

impl<T> MutVec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    /// The equivalent of `self[index]` -- load the element at the
    /// given index, panicking if there is no such element.
    pub fn at(&self, index: usize) -> T
    where
        T: Clone,
    {
        self.get(index).unwrap()
    }

    /// Attempt to get the element at the given `index`, returning
    /// `None` if it is out of bounds.
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        let data = self.data.borrow();
        Some(data.get(index)?.clone())
    }

    /// Push `value` onto the end of the vector.
    pub fn push(&self, value: T) {
        let mut data = self.data.borrow_mut();
        data.push(value);
    }

    /// Pop a value from the end of the vector, if any.
    pub fn pop(&self) -> Option<T> {
        let mut data = self.data.borrow_mut();
        data.pop()
    }

    /// Iterate over the elements in `self`, cloning them as we go.
    ///
    /// Note that it is possible to mutate `self` during this
    /// iteration (for example, by pushing or popping elements onto
    /// it). Doing so may lead to surprising results but is not
    /// undefined behavior in any way.
    pub fn iter(&self) -> Iter<'_, T>
    where
        T: Clone,
    {
        Iter {
            vec: self,
            index: 0,
        }
    }

    /// Take ownership of our internal vector, replacing it with `v`.
    pub fn replace(&self, v: Vec<T>) -> Vec<T> {
        self.data.replace(v)
    }

    /// Take ownership of our internal vector, replacing it with an
    /// empty one.
    pub fn take(&self) -> Vec<T> {
        self.data.take()
    }
}

impl<T: Clone> Clone for MutVec<T> {
    fn clone(&self) -> Self {
        let vec = self.data.borrow().clone();
        MutVec::from(vec.clone())
    }
}

impl<A> std::iter::FromIterator<A> for MutVec<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let v: Vec<A> = iter.into_iter().collect();
        MutVec::from(v)
    }
}

impl<T> Default for MutVec<T> {
    fn default() -> Self {
        Self::from(Vec::new())
    }
}

impl<T> From<Vec<T>> for MutVec<T> {
    fn from(v: Vec<T>) -> MutVec<T> {
        MutVec {
            data: MCell::new(v),
        }
    }
}

pub struct Iter<'iter, T>
where
    T: Clone,
{
    vec: &'iter MutVec<T>,
    index: usize,
}

impl<'iter, T> Iterator for Iter<'iter, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let value = self.vec.get(self.index)?;
        self.index += 1;
        Some(value)
    }
}
