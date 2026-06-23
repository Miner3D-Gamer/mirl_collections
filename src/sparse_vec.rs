use mirl_extensions_core::ListLike;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// # Disclaimer:
/// **use [`get_latest_idx`](Self::get_latest_idx) after using [`push`](super::ListLikeHelper::push) or [`push_mut`](ListLike::push_mut) to get the idx at which the new item lives**
///
/// A data structure that uses a few vectors to simulate a hashmap without the slow hashing of a hashmap
///     - Insert: O(1)
///     - Removal: O(1)
///     - Accessing: O(1) to O(N)
///     - Stable idx: true
///
/// The caveat: It can only ever grow as removing all elements will only remove 1/3 of the actively used storage
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseVec<T> {
    /// The actually stored values
    ///
    /// IT IS NOT RECOMMENDED TO EDIT THIS MANUALLY
    pub values: Vec<T>,
    /// A list pointing where the values are stored
    ///
    /// IT IS NOT RECOMMENDED TO EDIT THIS MANUALLY
    pub value_indexes: Vec<usize>,
    /// The inverse of the pointer list
    ///
    /// IT IS NOT RECOMMENDED TO EDIT THIS MANUALLY
    pub indexes_indexes: Vec<usize>,
}
impl<T> Default for SparseVec<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            value_indexes: Vec::new(),
            indexes_indexes: Vec::new(),
        }
    }
}
impl<T: std::cmp::PartialEq> ListLike<T, usize> for SparseVec<T> {
    type Iterator<'a>
        = UnorderedSparseVecIter<'a, T>
    where
        T: 'a;

    type IteratorMut<'a>
        = UnorderedSparseVecIterMut<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        <&Self as IntoIterator>::into_iter(self)
    }
    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        <&mut Self as IntoIterator>::into_iter(self)
    }
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe {
            self.values
                .get_unchecked(*self.value_indexes.as_slice().get_unchecked(index))
        }
    }
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe {
            self.values
                .get_unchecked_mut(*self.value_indexes.as_slice().get_unchecked(index))
        }
    }
    fn push_mut(&mut self, value: T) -> &mut T {
        if self.values.len() + 1 > self.value_indexes.len() {
            self.value_indexes.push(self.values.len());
            self.indexes_indexes.push(self.values.len());
        }
        self.values.push_mut(value)
    }
    fn swap_values(&mut self, a: usize, b: usize) -> bool {
        let first = self.value_indexes[a];
        let second = self.value_indexes[b];
        if first > self.len() || second > self.len() {
            return false;
        }
        self.swap_internal(first, second);
        true
    }
    fn try_remove(&mut self, index: usize) -> Option<T> {
        let a = self.value_indexes[index];
        let b = self.values.len() - 1;
        if a >= self.values.len() || b >= self.values.len() {
            return None;
        }
        self.swap_internal(a, b);

        // [`self.values`](SparseVec::values) cannot be empty,
        // if it was [`self.swap_internal`](SparseVec::swap_internal) would have already panicked
        Some(unsafe { self.values.pop().unwrap_unchecked() })
    }
    fn len(&self) -> usize {
        self.values.len()
    }
    fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }
    /// This function shall not be called.
    ///
    /// The point of insertion is pushing all further idx to the right which contradicts the point of having stable idx in [`SparseVec`]
    // TODO: Implement this
    fn try_insert_mut(&mut self, _index: usize, _value: T) -> Option<&mut T> {
        unimplemented!("`try_insert_mut` cannot be used on a sparse vec")
    }
    fn try_replace(&mut self, index: usize, value: T) -> Option<T> {
        self.values
            .try_replace(*self.value_indexes.get(index)?, value)
    }
    fn try_reserve(&mut self, amount: usize) -> Result<(), std::collections::TryReserveError> {
        self.values.try_reserve(amount)?;
        self.value_indexes.try_reserve(amount)?;
        self.indexes_indexes.try_reserve(amount)
    }
    fn find_position(&self, item: &T) -> Option<usize> {
        Some(self.indexes_indexes[self.values.find_position(item)?])
    }
    fn clear(&mut self) {
        self.values.clear();
        self.value_indexes.clear();
        self.indexes_indexes.clear();
    }
}

impl<T> SparseVec<T> {
    /// This swaps two values directly without first getting the value positions first
    pub fn swap_internal(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.values.len(), "Index a out of bounds");
        debug_assert!(b < self.values.len(), "Index b out of bounds");

        self.values.swap(a, b);
        self.indexes_indexes.swap(a, b);
        self.value_indexes[self.indexes_indexes[b]] = b;
        self.value_indexes[self.indexes_indexes[a]] = a;
    }
    #[must_use]
    /// After using [`push`](super::ListLikeHelper::push) or [`push_mut`](ListLike::push_mut), use this function to get the index of the pushed item
    pub fn get_latest_idx(&self) -> usize {
        self.indexes_indexes[self.values.len() - 1]
    }
}

impl<T> std::ops::Index<usize> for SparseVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[self.value_indexes[index]]
    }
}

impl<T> std::ops::IndexMut<usize> for SparseVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[self.value_indexes[index]]
    }
}
#[derive(Debug, Clone, Default)]
/// An iter over [`SparseVec`] that doesn't retain insertion order
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct UnorderedSparseVecIter<'a, T> {
    /// The inner iterator
    ///
    /// Not intended to be accessed
    pub inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for UnorderedSparseVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
#[derive(Debug, Default)]
/// A mutable iter over [`SparseVec`] that doesn't retain insertion order
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct UnorderedSparseVecIterMut<'a, T> {
    /// The inner iterator
    ///
    /// Not intended to be accessed
    pub inner: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for UnorderedSparseVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> SparseVec<T> {
    #[must_use]
    /// Iter over the internal values, the iter idx is not the same as the idx used to derive a value
    pub fn iter(&self) -> UnorderedSparseVecIter<'_, T> {
        UnorderedSparseVecIter {
            inner: self.values.iter(),
        }
    }

    #[must_use]
    /// Iter over the internal values mutably, the iter idx is not the same as the idx used to derive a value
    pub fn iter_mut(&mut self) -> UnorderedSparseVecIterMut<'_, T> {
        UnorderedSparseVecIterMut {
            inner: self.values.iter_mut(),
        }
    }
}

impl<'a, T> IntoIterator for &'a SparseVec<T> {
    type Item = &'a T;
    type IntoIter = UnorderedSparseVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SparseVec<T> {
    type Item = &'a mut T;
    type IntoIter = UnorderedSparseVecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> IntoIterator for SparseVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
