#![allow(clippy::type_complexity)]

pub mod iter;

//use std::collections::hash_map::{RandomState, Keys, Values, ValuesMut, Iter};
use std::collections::hash_map::{RandomState};
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

use core::hash::{BuildHasher, Hash};
//use ahash::RandomState;
use async_std::sync::RwLock;
use async_std::task;
use std::sync::Arc;
pub use iter::Iter;
//use std::borrow::Borrow;

pub struct AsyncHashMap<K, V, S = RandomState>
where
    K: Eq + Hash + Send,
    V: Send,
    S: BuildHasher + Default + Send,
{
    data: Arc<Box<RwLock<HashMap<K, V, S>>>>,
}

impl<K, V, S> Clone for AsyncHashMap<K, V, S>
    where
        K: Eq + Hash + Send + Clone,
        V: Send + Clone,
        S: BuildHasher + Default + Send + Clone,
{
    fn clone(&self) -> Self {
        let rwlock = self.data.clone();
        Self {
            data: rwlock,
        }
    }
}

impl<K, V> AsyncHashMap<K, V, RandomState>
where
    K: Eq + Hash + Send,
    V: Send,
{
    /// Creates an empty `AsyncHashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    /// let mut map: AsyncHashMap<&str, i32> = AsyncHashMap::new();
    /// ```
    #[inline]
    pub fn new() -> AsyncHashMap<K, V> {
        AsyncHashMap {
            data: Arc::new(Box::new(RwLock::new(HashMap::new()))),
        }
    }

    /// Creates an empty `AsyncHashMap` with the specified capacity.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    /// let mut map: AsyncHashMap<&str, i32> = AsyncHashMap::with_capacity(10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> AsyncHashMap<K, V, RandomState> {
        AsyncHashMap {
            data: Arc::new(Box::new(RwLock::new(HashMap::with_capacity(capacity)))),
        }
    }
}

impl<K, V, S> AsyncHashMap<K, V, S>
where
    K: Eq + Hash + Send + Sync,
    V: Send + Sync,
    S: BuildHasher + Default + Send + Sync + Clone,
{
    /// Creates an empty `AsyncHashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it
    /// is first inserted into.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    /// let mut map: AsyncHashMap<&str, i32> = AsyncHashMap::new();
    /// ```

    #[inline]
    pub fn with_hasher(hash_builder: S) -> AsyncHashMap<K, V, S> {
        AsyncHashMap {
            data: Arc::new(Box::new(RwLock::new(HashMap::with_hasher(hash_builder)))),
        }
    }

    /// Creates an empty `AsyncHashMap` with the specified capacity, using `hash_builder`
   /// to hash the keys.
   ///
   /// The hash map will be able to hold at least `capacity` elements without
   /// reallocating. If `capacity` is 0, the hash map will not allocate.
   ///
   /// Warning: `hash_builder` is normally randomly generated, and
   /// is designed to allow AsyncHashMaps to be resistant to attacks that
   /// cause many collisions and very poor performance. Setting it
   /// manually using this function can expose a DoS attack vector.
   ///
   /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
   /// the AsyncHashMap to be useful, see its documentation for details.
   ///
   /// # Examples
   ///
   /// ```
   /// use async_collection::AsyncHashMap;
   /// use std::collections::hash_map::RandomState;
   ///
   /// let s = RandomState::new();
   /// let mut map = AsyncHashMap::with_capacity_and_hasher(10, s);
   /// map.insert(1, 2);
   /// ```
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> AsyncHashMap<K, V, S> {
        AsyncHashMap {
            data: Arc::new(Box::new(RwLock::new(HashMap::with_capacity_and_hasher(capacity, hash_builder)))),
        }
    }

    #[inline]
    pub async fn capacity(&self) -> usize {
        let hashmap = self.data.read().await;
        hashmap.deref().capacity()
    }

    /// An iterator visiting all keys in arbitrary order.
   /// The iterator element type is `&'a K`.
   ///
   /// # Examples
   ///
   /// ```
   /// use async_collection::AsyncHashMap;
   ///
   /// let mut map = AsyncHashMap::new();
   /// map.insert("a", 1);
   /// map.insert("b", 2);
   /// map.insert("c", 3);
   ///
   /// for key in map.keys() {
   ///     println!("{}", key);
   /// }
   /// ```
   //  pub async fn keys(&self) -> &mut Keys<'a, K, V> {
   //      let AsyncHashMap = self.data.read().await;
   //      AsyncHashMap.deref().keys()
   //  }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    ///
    /// let mut map = AsyncHashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    /// }
    /// ```
    // pub async fn values(&self) -> Values<'_, K, V> {
    //     let hashmap = self.data.read().await;
    //     hashmap.deref().values()
    // }

    /// An iterator visiting all values mutably in arbitrary order.
    /// The iterator element type is `&'a mut V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    ///
    /// let mut map = AsyncHashMap::new();
    ///
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for val in map.values_mut() {
    ///     *val = *val + 10;
    /// }
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    /// }
    /// ```
    // pub async fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
    //     let hashmap = self.data.read().await;
    //     hashmap.deref().values_mut()
    // }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    ///
    /// let mut map = AsyncHashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    // pub async fn iter(&self) -> Iter<'_, K, V> {
    //     let hashmap = self.data.read().await;
    //     hashmap.deref().iter()
    // }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_collection::AsyncHashMap;
    ///
    /// let mut a = AsyncHashMap::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a");
    /// assert_eq!(a.len(), 1);
    /// ```
    pub async fn len(&self) -> usize {
        let hashmap = self.data.read().await;
        hashmap.len()
    }

    #[inline]
    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let mut hashmap = self.data.write().await;
        hashmap.deref_mut().insert(key, value)
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    pub async fn iter(&self) -> Iter<'_, K, V, S> {
        Iter::new(&self).await
    }
}

impl<K, V, S> Debug for AsyncHashMap<K, V, S>
where
    K: Eq + Hash + Send + Debug,
    V: Send + Debug,
    S: BuildHasher + Default + Send,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

pub struct AsyncHashSet<K, S = RandomState> {
    data: RwLock<HashSet<K, S>>,
}

impl<K, S> AsyncHashSet<K, S> {
    pub fn with_hasher(hash_builder: S) -> AsyncHashSet<K, S> {
        AsyncHashSet {
            data: RwLock::new(HashSet::with_hasher(hash_builder)),
        }
    }
}

// #[cfg(test)]
// mod tests_sync_hashmap {
//     #[test]
//     fn it_works() {
//         let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
//         println!("{}", map.capacity());
//         if let Some(re) = map.insert(1, 1) {
//             println!("old value:{}", re);
//         } else {
//             println!("insert new value");
//         }
//         if let Some(re) = map.insert(1, 6) {
//             println!("old value:{}", re);
//         } else {
//             println!("insert new value");
//         }
//         println!("{}", map.capacity());
//     }
// }

#[test]
fn test_new() {
    let hashmap: AsyncHashMap<i32, i32> = AsyncHashMap::new();
    let cap = task::block_on(async {
        if let Some(re) = hashmap.insert(4, 2).await {
            println!("old value:{}", re);
        } else {
            println!("insert new value");
        }
        if let Some(re) = hashmap.insert(4, 3).await {
            println!("old value:{}", re);
        } else {
            println!("insert new value");
        }
        hashmap.capacity().await
    });
    println!("{}", cap);
    assert_ne!(cap, 0)
}

#[test]
fn test_insert() {
    let hashmap: AsyncHashMap<i32, i32> = AsyncHashMap::new();

    let len = task::block_on(async {
        task::spawn(async {
            hashmap.insert(9, 1).await;
            hashmap.insert(6, 1).await;
            hashmap.insert(7, 1).await;
            let len = hashmap.len().await;
            println!("test_insert3 len:{}", len);
        });

        hashmap.insert(4, 2).await;
        hashmap.insert(5, 3).await;



        let rwhashmap = hashmap.data.read().await;
        for (k, v) in rwhashmap.iter() {
            println!("key:{}, value:{}", k, v);
        }

        hashmap.len().await
    });

    task::spawn(async move {
        hashmap.insert(9, 1).await;
        hashmap.insert(6, 1).await;
        hashmap.insert(7, 1).await;
        let len = hashmap.len().await;
        println!("test_insert3 len:{}", len);
    });

    println!("test_insert2 len:{}", len);
    assert_eq!(len, 2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
