#![allow(deprecated)]
use mirl_extensions_core::ListLike;

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[deprecated = "This struct is not yet fully implemented. `.pop()`, `.insert_mut()`, and `.try_replace()` are not available and will panic."]
/// A Vec type with at least one element

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyVec<T> {
    /// The must-have value
    pub _first: T,
    /// The rest
    pub _rest: Vec<T>,
}
impl<T> NonEmptyVec<T> {
    /// Create a new [`NonEmptyVec`] with a single item inside
    pub const fn new(item: T) -> Self {
        Self {
            _first: item,
            _rest: Vec::new(),
        }
    }
}
#[allow(clippy::into_iter_without_iter)] // The trait implements iter
impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;

    type IntoIter = NonEmptyVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        NonEmptyVecIter {
            inner: self._rest.iter(),

            inner_first: Some(&self._first),
        }
    }
}
#[allow(clippy::into_iter_without_iter)] // The trait implements iter
impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;

    type IntoIter = NonEmptyVecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        NonEmptyVecIterMut {
            inner: self._rest.iter_mut(),

            inner_first: Some(&mut self._first),
        }
    }
}
impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;

    type IntoIter = IntoNonEmptyVecIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoNonEmptyVecIter {
            inner: self._rest.into_iter(),

            inner_first: Some(self._first),
        }
    }
}

const impl<T: [const] std::default::Default> std::default::Default for NonEmptyVec<T> {
    fn default() -> Self {
        Self {
            _first: T::default(),
            _rest: Vec::new(),
        }
    }
}
impl<T> std::ops::IndexMut<usize> for NonEmptyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self._first
        } else {
            self._rest.index_mut(index - 1)
        }
    }
}
impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self._first
        } else {
            self._rest.index(index - 1)
        }
    }
}

impl<T> ListLike<T, usize> for NonEmptyVec<T> {
    // type Iterator<'a>
    //     = NonEmptyVecIter<'a, T>
    // where
    //     T: 'a;

    // type IteratorMut<'a>
    //     = NonEmptyVecIterMut<'a, T>
    // where
    //     T: 'a;
    // fn iter(&self) -> Self::Iterator<'_> {
    //     <&Self as IntoIterator>::into_iter(self)
    // }
    // fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
    //     <&mut Self as IntoIterator>::into_iter(self)
    // }
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        if index == 0 {
            &self._first
        } else {
            unsafe { self._rest.get_unchecked(index - 1) }
        }
    }
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        if index == 0 {
            &mut self._first
        } else {
            unsafe { self._rest.get_unchecked_mut(index - 1) }
        }
    }

    fn push_mut(&mut self, value: T) -> &mut T {
        self._rest.push_mut(value)
    }
    /// Try to remove a value. If you try to remove the last value None is returned instead
    fn try_remove(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            if self._rest.is_empty() {
                None
            } else {
                let new_first = unsafe { self._rest.try_remove(0).unwrap_unchecked() };
                Some(std::mem::replace(&mut self._first, new_first))
            }
        } else {
            self._rest.try_remove(index - 1)
        }
    }

    fn swap_values(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = if a > b { (b, a) } else { (a, b) };
        if a == b {
            return true;
        }
        if b > self._rest.len() {
            return false;
        }
        if a == 0 {
            unsafe {
                std::mem::swap(&mut self._first, self._rest.get_unchecked_mut(b - 1));
            }
        }
        if a > self._rest.len() {
            return false;
        }
        unsafe {
            self._rest.swap_unchecked(a, b);
        }
        true
    }

    fn len(&self) -> usize {
        self._rest.len() + 1
    }

    fn pop(&mut self) -> Option<T> {
        self._rest.pop()
    }
    // TODO: This could be outputting the wrong &mut T if index is zero, idk
    fn try_insert_mut(&mut self, index: usize, value: T) -> Option<&mut T> {
        if index == 0 {
            // Safety: Index 0 is always valid
            let first = unsafe { self.try_replace(0, value).unwrap_unchecked() };
            self._rest.try_insert_mut(0, first)
        } else {
            self._rest.try_insert_mut(index - 1, value)
        }
    }

    fn try_replace(&mut self, index: usize, value: T) -> Option<T> {
        if index == 0 {
            let mut output = value;
            std::mem::swap(&mut output, &mut self._first);
            Some(output)
        } else {
            self._rest.try_replace(index, value)
        }
    }

    fn try_reserve(&mut self, amount: usize) -> Result<(), std::collections::TryReserveError> {
        self._rest.try_reserve(amount.saturating_sub(1))
    }

    fn find_position(&self, item: &T) -> Option<usize>
    where
        T: std::cmp::PartialEq,
    {
        if self._first.eq(item) {
            Some(0)
        } else {
            self._rest.find_position(item).map(|x| x - 1)
        }
    }
    /// Clears every value but the first.
    fn clear(&mut self) {
        self._rest.clear();
    }
}

#[derive(Debug, Clone)]
/// An iter over [`NonEmptyVec`]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct NonEmptyVecIter<'a, T> {
    /// The inner iterator
    ///
    /// Not intended to be accessed
    pub inner: std::slice::Iter<'a, T>,

    /// The first element
    ///
    /// Not intended to be accessed
    pub inner_first: Option<&'a T>,
}

impl<'a, T> Iterator for NonEmptyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner_first.is_some() {
            std::mem::take(&mut self.inner_first)
        } else {
            self.inner.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.inner.size_hint();
        // TODO: Check if it wants the total or remaining items in the iter
        if self.inner_first.is_some() {
            (hint.0 + 1, hint.1.map(|x| x + 1))
        } else {
            hint
        }
    }
}
#[derive(Debug)]
/// A mutable iter over [`NonEmptyVec`]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct NonEmptyVecIterMut<'a, T> {
    /// The inner iterator
    ///
    /// Not intended to be accessed
    pub inner: std::slice::IterMut<'a, T>,

    /// The first element
    ///
    /// Not intended to be accessed
    pub inner_first: Option<&'a mut T>,
}

impl<'a, T> Iterator for NonEmptyVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner_first.is_some() {
            std::mem::take(&mut self.inner_first)
        } else {
            self.inner.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.inner.size_hint();
        // TODO: Check if it wants the total or remaining items in the iter
        if self.inner_first.is_some() {
            (hint.0 + 1, hint.1.map(|x| x + 1))
        } else {
            hint
        }
    }
}

#[derive(Debug, Clone)]
/// An iter over [`NonEmptyVec`]
#[cfg_attr(feature = "c_compatible", repr(C))]
pub struct IntoNonEmptyVecIter<T> {
    /// The inner iterator
    ///
    /// Not intended to be accessed
    pub inner: std::vec::IntoIter<T>,

    /// The first element
    ///
    /// Not intended to be accessed
    pub inner_first: Option<T>,
}

impl<T> Iterator for IntoNonEmptyVecIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner_first.is_some() {
            std::mem::take(&mut self.inner_first)
        } else {
            self.inner.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.inner.size_hint();
        // TODO: Check if it wants the total or remaining items in the iter
        if self.inner_first.is_some() {
            (hint.0 + 1, hint.1.map(|x| x + 1))
        } else {
            hint
        }
    }
}
