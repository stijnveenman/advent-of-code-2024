use std::collections::{hash_map::Entry, HashMap};

use debug_iterator::{debug_iterator, DebugIter};
use u32_iterator::{u32_iterator, U32Iter};
use usize_iterator::{usize_iterator, UsizeIter};

pub mod debug_iterator;
mod u32_iterator;
mod usize_iterator;

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

    fn usize(self) -> UsizeIter<Self>
    where
        Self: Sized,
    {
        usize_iterator(self)
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

    fn find_by_max<F: Fn(&Self::Item) -> usize>(self, map_fn: F) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let mut current = None;
        let mut max = usize::MIN;

        for c in self {
            let v = map_fn(&c);

            if v > max {
                max = v;
                current = Some(c);
            }
        }

        current
    }
}

impl<T> AocItertools for T where T: Iterator + ?Sized {}
