use std::collections::{hash_map::Entry, HashMap};

use debug_iterator::{debug_iterator, DebugIter};
use u32_iterator::{u32_iterator, U32Iter};

pub mod debug_iterator;
mod u32_iterator;

pub trait AocItertools: Iterator {
    fn dbg(self) -> DebugIter<Self>
    where
        Self: Sized,
    {
        debug_iterator(self)
    }

    fn u32(self) -> U32Iter<Self>
    where
        Self: Sized,
    {
        u32_iterator(self)
    }

    fn grouped_by<TKey, T: Fn(&Self::Item) -> TKey>(
        self,
        key_fn: T,
    ) -> HashMap<TKey, Vec<<Self as Iterator>::Item>>
    where
        Self: Sized,
        TKey: std::cmp::Eq + std::hash::Hash,
    {
        let mut map: HashMap<TKey, Vec<<Self as Iterator>::Item>> = HashMap::new();

        for v in self {
            let key = key_fn(&v);

            match map.entry(key) {
                Entry::Occupied(mut entry) => entry.get_mut().push(v),
                Entry::Vacant(entry) => {
                    entry.insert(vec![v]);
                }
            };
        }

        map
    }
}

impl<T> AocItertools for T where T: Iterator + ?Sized {}
