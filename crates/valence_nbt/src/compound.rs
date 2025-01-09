extern crate alloc;

use alloc::borrow::{Borrow, Cow};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt;
use core::hash::Hash;
use core::iter::FusedIterator;
use core::ops::{Index, IndexMut};
use crate::Value;

/// A map type with [`String`] keys and [`Value`] values.
#[derive(Clone, Default)]
pub struct Compound<S = String> {
    map: Map<S>,
}

type Map<S> = BTreeMap<S, Value<S>>;

impl<S: fmt::Debug> fmt::Debug for Compound<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.map.fmt(f)
    }
}

impl<S> PartialEq for Compound<S>
where
    S: Ord + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<S> Compound<S> {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }

    pub fn with_capacity(_cap: usize) -> Self {
        // BTreeMap does not support capacity
        Self::new()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl<S> Compound<S>
where
    S: Ord + Hash,
{
    pub fn get<Q>(&self, k: &Q) -> Option<&Value<S>>
    where
        Q: ?Sized + AsBorrowed<S>,
        <Q as AsBorrowed<S>>::Borrowed: Hash + Ord,
        S: Borrow<<Q as AsBorrowed<S>>::Borrowed>,
    {
        self.map.get(k.as_borrowed())
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Q: ?Sized + AsBorrowed<S>,
        <Q as AsBorrowed<S>>::Borrowed: Hash + Ord,
        S: Borrow<<Q as AsBorrowed<S>>::Borrowed>,
    {
        self.map.contains_key(k.as_borrowed())
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut Value<S>>
    where
        Q: ?Sized + AsBorrowed<S>,
        <Q as AsBorrowed<S>>::Borrowed: Hash + Ord,
        S: Borrow<<Q as AsBorrowed<S>>::Borrowed>,
    {
        self.map.get_mut(k.as_borrowed())
    }

    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<Value<S>>
    where
        K: Into<S>,
        V: Into<Value<S>>,
    {
        self.map.insert(k.into(), v.into())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// Trait that can be used as a key to query a compound. Basically something
/// that can be converted to a type `B` such that `S: Borrow<B>`.
pub trait AsBorrowed<S> {
    type Borrowed: ?Sized;

    fn as_borrowed(&self) -> &Self::Borrowed;
}

impl<Q: ?Sized> AsBorrowed<String> for Q
where
    String: Borrow<Q>,
{
    type Borrowed = Q;

    #[inline]
    fn as_borrowed(&self) -> &Q {
        self
    }
}

impl<'a, Q: ?Sized> AsBorrowed<Cow<'a, str>> for Q
where
    Cow<'a, str>: Borrow<Q>,
{
    type Borrowed = Q;

    #[inline]
    fn as_borrowed(&self) -> &Q {
        self
    }
}

impl<S> FromIterator<(S, Value<S>)> for Compound<S>
where
    S: Ord + Hash,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (S, Value<S>)>,
    {
        Self {
            map: Map::from_iter(iter),
        }
    }
}

impl<S, Q> Index<&'_ Q> for Compound<S>
where
    S: Borrow<Q> + Ord + Hash,
    Q: ?Sized + Ord + Hash,
{
    type Output = Value<S>;

    fn index(&self, index: &Q) -> &Self::Output {
        self.map.index(index)
    }
}

impl<S, Q> IndexMut<&'_ Q> for Compound<S>
where
    S: Borrow<Q> + Hash + Ord,
    Q: ?Sized + Ord + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

/// Implement standard iterator traits for Compound
impl<'a, S> IntoIterator for &'a Compound<S> {
    type Item = (&'a S, &'a Value<S>);
    type IntoIter = core::collections::btree_map::Iter<'a, S, Value<S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

/// Key iterator for Compound
#[derive(Clone, Debug)]
pub struct Keys<'a, S> {
    iter: core::collections::btree_map::Keys<'a, S, Value<S>>,
}

impl<'a, S> Iterator for Keys<'a, S> {
    type Item = &'a S;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Value iterator for Compound
#[derive(Clone, Debug)]
pub struct Values<'a, S> {
    iter: core::collections::btree_map::Values<'a, S, Value<S>>,
}

impl<'a, S> Iterator for Values<'a, S> {
    type Item = &'a Value<S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
