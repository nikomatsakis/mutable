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

use crate::mutbl::Mut;
use crate::pure_clone::PureClone;
use std::vec::Vec;

impl<T> Mut<Vec<T>> {
    pub fn len(&self) -> usize {
        let vec: *mut Vec<T> = self.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &Vec<T> = unsafe { &mut *vec };

        vec.len()
    }

    pub fn at(&self, index: usize) -> Option<T>
    where
        T: PureClone,
    {
        let vec: *mut Vec<T> = self.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &Vec<T> = unsafe { &mut *vec };

        Some(vec.get(index)?.clone())
    }

    pub fn push(&self, value: T) {
        let vec: *mut Vec<T> = self.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &mut Vec<T> = unsafe { &mut *vec };

        vec.push(value);
    }

    pub fn pop(&self) -> Option<T> {
        let vec: *mut Vec<T> = self.as_ptr();

        // Unsafe obligation: See [1] above
        let vec: &mut Vec<T> = unsafe { &mut *vec };

        vec.pop()
    }

    pub fn iter(&self) -> MutVecIter<'_, T>
    where T: PureClone
    {
        let length = self.len();
        MutVecIter { vec: self, index: 0, length }
    }
}

pub struct MutVecIter<'iter, T>
where
    T: PureClone,
{
    vec: &'iter Mut<Vec<T>>,
    index: usize,
    length: usize,
}

impl<'iter, T> Iterator for MutVecIter<'iter, T> where T: PureClone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index < self.length {
            let i = self.index;
            self.index += 1;
            Some(self.vec.at(i).unwrap())
        } else {
            None
        }
    }
}
