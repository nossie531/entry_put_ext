//! Provider of [`EntryPutExt`].

use std::collections::btree_map::Entry::{self, *};

/// Extension trait of [`Entry`] for put operations.
pub trait EntryPutExt<'a, V>
where
    V: 'a,
{
    /// Sets the value of the entry, and returns a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use entry_put_ext::btree_map::EntryPutExt;
    ///
    /// let mut map = BTreeMap::from([("X", false)]);
    /// let x = *map.entry("X").put(true);
    /// let y = *map.entry("Y").put(true);
    ///
    /// assert_eq!(x, map["X"]);
    /// assert_eq!(y, map["Y"]);
    /// ```
    fn put(self, v: V) -> &'a mut V;
}

impl<'a, K, V> EntryPutExt<'a, V> for Entry<'a, K, V>
where
    K: Ord,
{
    fn put(self, v: V) -> &'a mut V {
        match self {
            Vacant(x) => x.insert(v),
            Occupied(x) => {
                let vr = x.into_mut();
                *vr = v;
                vr
            }
        }
    }
}
