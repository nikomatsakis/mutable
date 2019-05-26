use crate::mcell::MCell;
use indexmap::Equivalent;
use indexmap::IndexMap;
use std::hash::Hash;

mod test;

pub struct MutMap<K, V> {
    data: MCell<IndexMap<K, V>>,
}

impl<K, V> MutMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    /// Inserts `(key, value)` into the map. During the insertion
    /// operation, all mut-cells are locked and read-only. Attempts to
    /// read from *this* map during insertion will encounter an empty
    /// map.
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.data.check_out().insert(key, value)
    }

    /// Removes `key` from the map. During the removal operation, all
    /// mut-cells are locked and read-only. Attempts to read from
    /// *this* map during removal will encounter an empty map.
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K>,
    {
        self.data.check_out().remove(key)
    }

    /// A variant on `insert` where all data is lost on panic. This
    /// exists for benchmarking purposes.
    pub fn insert_not_panic_safe(&self, key: K, value: V) -> Option<V> {
        self.data
            .check_out_not_panic_safe(|data| data.insert(key, value))
    }

    /// A variant on `insert` where all data is lost on panic. This
    /// exists for benchmarking purposes.
    pub fn remove_not_panic_safe<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K>,
    {
        self.data.check_out_not_panic_safe(|data| data.remove(key))
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<V>
    where
        Q: Hash + Equivalent<K>,
        V: Clone,
    {
        let data = self.data.borrow();
        data.get(key).cloned()
    }

    pub fn get_index(&self, index: usize) -> Option<(K, V)>
    where
        K: Clone,
        V: Clone,
    {
        let data = self.data.borrow();
        let (k, v) = data.get_index(index)?;
        Some((k.clone(), v.clone()))
    }

    pub fn get_key_index(&self, index: usize) -> Option<K>
    where
        K: Clone,
    {
        let data = self.data.borrow();
        let (k, _) = data.get_index(index)?;
        Some(k.clone())
    }

    pub fn get_value_index(&self, index: usize) -> Option<V>
    where
        V: Clone,
    {
        let data = self.data.borrow();
        let (_, v) = data.get_index(index)?;
        Some(v.clone())
    }

    /// Iterate over the elements in `self`, cloning them as we go.
    ///
    /// Note that it is possible to mutate `self` during this
    /// iteration (for example, by pushing or popping elements onto
    /// it). Doing so may lead to surprising results but is not
    /// undefined behavior in any way.
    pub fn iter(&self) -> Iter<'_, K, V>
    where
        K: Clone,
        V: Clone,
    {
        Iter {
            map: self,
            index: 0,
        }
    }

    /// Iterate over the elements in `self`, cloning them as we go.
    ///
    /// Note that it is possible to mutate `self` during this
    /// iteration (for example, by pushing or popping elements onto
    /// it). Doing so may lead to surprising results but is not
    /// undefined behavior in any way.
    pub fn keys(&self) -> Keys<'_, K, V>
    where
        K: Clone,
    {
        Keys {
            map: self,
            index: 0,
        }
    }
}

impl<K: Clone, V: Clone> Clone for MutMap<K, V>
where
    K: Eq + Hash,
{
    fn clone(&self) -> Self {
        let map = self.data.borrow().clone();
        MutMap::from(map.clone())
    }
}

impl<K, V> std::iter::FromIterator<(K, V)> for MutMap<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let v: IndexMap<K, V> = iter.into_iter().collect();
        MutMap::from(v)
    }
}

impl<K, V> Default for MutMap<K, V> {
    fn default() -> Self {
        Self::from(IndexMap::new())
    }
}

impl<K, V> From<IndexMap<K, V>> for MutMap<K, V> {
    fn from(v: IndexMap<K, V>) -> MutMap<K, V> {
        MutMap {
            data: MCell::new(v),
        }
    }
}

pub struct Iter<'iter, K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    map: &'iter MutMap<K, V>,
    index: usize,
}

impl<'iter, K, V> Iterator for Iter<'iter, K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        let (key, value) = self.map.get_index(self.index)?;
        self.index += 1;
        Some((key, value))
    }
}

pub struct Keys<'iter, K, V>
where
    K: Eq + Hash + Clone,
{
    map: &'iter MutMap<K, V>,
    index: usize,
}

impl<'iter, K, V> Iterator for Keys<'iter, K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = K;

    fn next(&mut self) -> Option<K> {
        let key = self.map.get_key_index(self.index)?;
        self.index += 1;
        Some(key)
    }
}
