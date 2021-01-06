#![allow(clippy::type_complexity)]

//use std::sync::Arc;
use core::hash::{BuildHasher, Hash};
//use std::collections::hash_map::{Keys as SynKeys, Values as SynValues};
use std::collections::hash_map::{RandomState, Iter as SyncIter};
//use std::fmt::{self, Debug};

//use async_std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use async_std::sync::RwLockReadGuard;

use crate::{AsyncHashMap};
use std::collections::{HashMap};
//use std::ops::Deref;
//use std::ops::Deref;

pub struct Iter<'a, K, V, S = RandomState>
    where
        K: 'a + Eq + Hash + Send + Sync,
        V: 'a + Send + Sync,
        S: 'a + BuildHasher + Clone + Default + Send + Sync,
{
     map: &'a AsyncHashMap<K, V, S>,
     rwlock: RwLockReadGuard<'a, HashMap<K, V, S>>,
     //iter: SyncIter<'a, K, V>,
}

unsafe impl<'a, 'i, K, V, S> Send for Iter<'i, K, V, S>
    where
        K: 'a + Eq + Hash + Send + Sync,
        V: 'a + Send + Sync,
        S: 'a + BuildHasher + Clone + Default + Send + Sync,
{
}

unsafe impl<'a, 'i, K, V, S> Sync for Iter<'i, K, V, S>
    where
        K: 'a + Eq + Hash + Send + Sync,
        V: 'a + Send + Sync,
        S: 'a + BuildHasher + Clone + Default + Send + Sync,
{
}

impl<'a, 'b, K: Eq + Hash, V, S: 'a + BuildHasher + Clone> Iter<'a, K, V, S>
    where
    K: 'a + Eq + Hash + Send + Sync,
    V: 'a + Send + Sync,
    S: 'a + BuildHasher + Clone + Default + Send + Sync,
{
    //, readlock: &'a RwLockReadGuard<'a, HashMap<K, V, S>>
    pub(crate) async fn new(map: &'a AsyncHashMap<K, V, S>) -> Iter<'a, K, V, S> {
        //let guard = Box::new(map.data.read().await);
        //let iter = guard.deref().deref().iter();
        // let rwlockl = map.data.read().await;
        // let iterl = rwlockl.iter();
        Self {
            map,
            rwlock: map.data.read().await,
            //iter: iterl,
        }
    }

    pub(crate) fn iter(&'a self) -> SyncIter<'a, K, V> {
        self.rwlock.iter()
    }
}

// impl<'a, K: Eq + Hash, V, S: 'a + BuildHasher + Clone> Iterator
// for Iter<'a, K, V, S>
//     where
//     K: 'a + Eq + Hash + Send + Sync,
//     V: 'a + Send + Sync,
//     S: 'a + BuildHasher + Clone + Default + Send + Sync,
// {
//     type Item = (&'a K, &'a V);
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some((k, v)) = self.rwlock.iter.next() {
//             return Some((k, v));
//         }
//         None
//     }
// }

// type GuardIterMut<'a, K, V, S> = (
//     Arc<RwLockWriteGuard<'a, AsyncHashMap<K, V, S>>>,
//     AsyncHashMap::IterMut<'a, K, SharedValue<V>>,
// );
//
// pub struct IterMut<'a, K, V, S = RandomState, M = AsyncHashMap<K, V, S>> {
//     map: &'a M,
//     current: Option<GuardIterMut<'a, K, V, S>>,
// }
//
// unsafe impl<'a, 'i, K, V, S, M> Send for IterMut<'i, K, V, S, M>
//     where
//         K: 'a + Eq + Hash + Send,
//         V: 'a + Send,
//         S: 'a + BuildHasher + Clone,
//         M: Map<'a, K, V, S>,
// {
// }
//
// unsafe impl<'a, 'i, K, V, S, M> Sync for IterMut<'i, K, V, S, M>
//     where
//         K: 'a + Eq + Hash + Sync,
//         V: 'a + Sync,
//         S: 'a + BuildHasher + Clone,
//         M: Map<'a, K, V, S>,
// {
// }
//
// /// An iterator over the keys of a `AsyncHashMap`.
// ///
// /// This `struct` is created by the [`keys`] method on [`AsyncHashMap`]. See its
// /// documentation for more.
// ///
// /// [`keys`]: AsyncHashMap::keys
// pub struct Keys<'a, K: 'a, V: 'a> {
//
//     inner: AsyncKeys<'a, K, V>,
// }
//
// impl<K, V> Clone for Keys<'_, K, V> {
//     #[inline]
//     fn clone(&self) -> Self {
//         Keys { inner: self.inner.clone() }
//     }
// }
//
// impl<K: Debug, V> fmt::Debug for Keys<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.clone()).finish()
//     }
// }
//
// /// An iterator over the values of a `AsyncHashMap`.
// ///
// /// This `struct` is created by the [`values`] method on [`AsyncHashMap`]. See its
// /// documentation for more.
// ///
// /// [`values`]: AsyncHashMap::values
// pub struct Values<'a, K: 'a, V: 'a> {
//     inner: Iter<'a, K, V>,
// }
//
// impl<K, V> Clone for Values<'_, K, V> {
//     #[inline]
//     fn clone(&self) -> Self {
//         Values { inner: self.inner.clone() }
//     }
// }
//
// impl<K, V: Debug> fmt::Debug for Values<'_, K, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_list().entries(self.clone()).finish()
//     }
// }
//
//
