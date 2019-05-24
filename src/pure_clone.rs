use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

/// A value `v` is "pure cloneable" if it can be cloned (via the
/// standard `Clone` impl) without mutating any cells that may
/// (transitively) own `v`. Typically, such clones are also O(1), but
/// this is not required.
#[marker]
pub unsafe trait PureClone: Clone {
}

unsafe impl<T: Copy> PureClone for T {}

// Interesting case: Rc *does* mutate a cell, but that cell does not
// own the rc (it contains the reference count).
unsafe impl<T> PureClone for Rc<T> {}

// Interesting case: Rc *does* mutate a cell, but that cell does not
// own the rc (it contains the reference count).
unsafe impl<T> PureClone for Arc<T> {}

// Interesting case: cloning the vector will cause its elements to be
// cloned.  These elements are owned by the vector. Since they are
// also "pure clone", they cannot mutate any cell that owns the
// vector, as that same cell would transitively own them.
unsafe impl<T: PureClone> PureClone for Vec<T> {}
unsafe impl<K: PureClone, V: PureClone> PureClone for HashMap<K, V> {}

