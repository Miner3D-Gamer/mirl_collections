use mirl_core::misc::vec_try_remove;
use mirl_extensions_core::IndexedMap;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "wincode",
    derive(wincode::SchemaRead, wincode::SchemaWrite)
)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A "`HashMap`" like struct that keeps the insert order and duplicate items but also doesn't require the items to implement `Hash` or `Ord`
pub struct VecMap<K, V, const KEEP_UNIQUE: bool> {
    /// The internal map
    pub map: Vec<(K, V)>,
}
impl<K: core::cmp::PartialEq, V> VecMap<K, V, false> {
    /// Push a value on the vec, always return None
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.map.push((key, val));
        None
    }
}
impl<K, V, const KEEP_UNIQUE: bool> const Default
    for VecMap<K, V, KEEP_UNIQUE>
{
    fn default() -> Self {
        Self {
            map: Vec::new(),
        }
    }
}
// impl<K: core::cmp::PartialEq, V, const KEEP_UNIQUE: bool>
//     std::iter::IntoIterator for VecMap<K, V, KEEP_UNIQUE>
// {
//     fn into_iter(self) -> Self::IntoIter {
//         self.map.iter().map(|x| (x.0, x.1))
//     }
//     type Item = (K, V);

//     type IntoIter = std::vec::IntoIter<Self::Item>;
// }

impl<K: core::cmp::PartialEq, V> VecMap<K, V, true> {
    /// Push a value on the vec, always return None
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let output = self.remove(&key);
        self.map.push((key, val));
        output
    }
}
impl<K: core::cmp::PartialEq, V, const KEEP_UNIQUE: bool>
    VecMap<K, V, KEEP_UNIQUE>
{
    /// Create a new empty [`VecMap`]
    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }
    #[must_use]
    /// Get a value mutably based on the key
    ///
    /// Complexity: O(N)
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.iter().find(|x| x.0.eq(key)).map(|x| &x.1)
    }
    #[must_use]
    /// Get a value based on the key
    ///
    /// Complexity: O(N)
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.iter_mut().find(|x| x.0.eq(key)).map(|x| &mut x.1)
    }

    /// Remove a value based on the key
    ///
    /// Complexity: O(N)
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.map.iter().position(|x| x.0.eq(key))?;
        vec_try_remove(&mut self.map, idx).map(|x| x.1)
    }

    /// Get an iterator over the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.iter().map(|x| &x.0)
    }
    /// Get an iterator over the items
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.iter().map(|x| &x.1)
    }
    /// Get an iterator over the list
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter().map(|x| (&x.0, &x.1))
    }
    /// Get an iterator over the list
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut K, &mut V)> {
        self.map.iter_mut().map(|x| (&mut x.0, &mut x.1))
    }
    #[must_use]
    /// Get a key based on the value
    ///
    /// Complexity: O(N)
    pub fn find_key(&self, value: &V) -> Option<&K>
    where
        V: PartialEq,
    {
        self.map.iter().find(|x| x.1.eq(value)).map(|x| &x.0)
    }
    #[must_use]
    /// Get the length of the list
    ///
    /// Complexity: O(1)
    pub const fn len(&self) -> usize {
        self.map.len()
    }
    #[must_use]
    /// Get the length of the list
    ///
    /// Complexity: O(1)
    pub const fn is_empty(&self) -> bool {
        self.map.len() == 0
    }
}
impl<K: core::cmp::PartialEq, V> IntoIterator for VecMap<K, V, false> {
    type Item = (K, V);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
impl<K: core::cmp::PartialEq, V> mirl_extensions_core::Map<K, V>
    for VecMap<K, V, false>
{
    fn insert(&mut self, key: K, val: V) -> Option<V> {
        self.insert(key, val)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a K, &'a V)> + '_> {
        Box::new(self.iter())
    }

    fn find_key(&self, value: &V) -> Option<&K>
    where
        V: PartialEq,
    {
        self.find_key(value)
    }

    fn len(&self) -> usize {
        self.len()
    }
}
impl<K: core::cmp::PartialEq, V> IndexedMap<K, V> for VecMap<K, V, false> {
    fn get_index(&self, index: usize) -> Option<&V> {
        self.map.get(index).map(|x| &x.1)
    }
    fn get_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.map.get_mut(index).map(|x| &mut x.1)
    }
    fn index(&self, value: &V) -> Option<usize>
    where
        V: PartialEq,
    {
        self.map.iter().position(|x| x.1.eq(value))
    }
}
