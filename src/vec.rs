// [1] Unsafe obligations: In each of these cases, we need to know
// that invoking the vec methods will never access this cell in any
// way (neither get nor set). This is in some sense a bit dubious
// because the stdlib doesn't really make any such promise. But of
// course we know that, in practice, it does not.
//
// (This, btw, is an interesting case where being able to reason about
// parametricity might help -- i.e., we know that `push` has very
// limited bounds, so we might be able to reason from those bounds --
// but of course, with specialization, Vec could dynamically detect
// new capabilities and make use of them.)

use crate::pure_clone::PureClone;
use std::vec::Vec;
use std::cell::Cell;

pub struct MutVec<T> {
    data: Cell<Vec<T>>
}

impl<T> MutVec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        let vec: *mut Vec<T> = self.data.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &Vec<T> = unsafe { &mut *vec };

        vec.len()
    }

    pub fn get(&self, index: usize) -> Option<T>
    where
        T: PureClone,
    {
        let vec: *mut Vec<T> = self.data.as_ptr();

        // Unsafe obligation: See [1] above; also, we know that
        // `T::clone` is "pure".
        let vec: &Vec<T> = unsafe { &mut *vec };

        Some(vec.get(index)?.clone())
    }

    pub fn push(&self, value: T) {
        let vec: *mut Vec<T> = self.data.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &mut Vec<T> = unsafe { &mut *vec };

        vec.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let vec: *mut Vec<T> = self.data.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &mut Vec<T> = unsafe { &mut *vec };

        vec.pop()
    }

    /// Iterate over the elements in `self`, cloning them as we go.
    ///
    /// Note that it is possible to mutate `self` during this
    /// iteration (for example, by pushing or popping elements onto
    /// it). Doing so may lead to surprising results but is not
    /// undefined behavior in any way.
    pub fn iter(&self) -> MutVecIter<'_, T>
    where T: PureClone
    {
        MutVecIter { vec: self, index: 0 }
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

impl<T: PureClone> Clone for MutVec<T> {
    fn clone(&self) -> Self {
        let vec: *mut Vec<T> = self.data.as_ptr();

        // Unsafe obligation: See [1] above; also, we know that
        // `T::clone` is "pure".
        let vec: &Vec<T> = unsafe { &mut *vec };

        MutVec::from(vec.clone())
    }
}

// Unsafe obligation: we know that `Vec<T>` is pure-clone, so cloning
// it (which is what we do...) cannot affect any of its transitive
// owners (us).
unsafe impl<T: PureClone> PureClone for MutVec<T> {
}

impl<A> std::iter::FromIterator<A> for MutVec<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>
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
        MutVec { data: Cell::new(v) }
    }
}

pub struct MutVecIter<'iter, T>
where
    T: PureClone,
{
    vec: &'iter MutVec<T>,
    index: usize,
}

impl<'iter, T> Iterator for MutVecIter<'iter, T> where T: PureClone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let value = self.vec.get(self.index)?;
        self.index += 1;
        Some(value)
    }
}

#[test]
fn iter1() {
    let v = MutVec::new();

    v.push(22);
    v.push(44);
    v.push(66);

    // Demonstrate iterating while also mutating the vector (in this
    // case, removing things from the end).
    let mut results = vec![];
    for i in v.iter() {
        results.push(Some(i));
        results.push(v.pop());
    }

    assert_eq!(
        results,
        vec![Some(22), Some(66), Some(44), Some(44)],
    );
}

