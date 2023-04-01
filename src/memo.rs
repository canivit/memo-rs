use std::hash::Hash;
use std::num::NonZeroUsize;

use crate::cache::LruCache;

pub fn create_memo<K, V>(func: fn(K) -> V, cache_capacity: NonZeroUsize) -> impl FnMut(K) -> V
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    let mut memo = Memo::new(func, cache_capacity);
    move |input| memo.call(input)
}

struct Memo<K, V> {
    func: fn(K) -> V,
    cache: LruCache<K, V>,
}

impl<K, V> Memo<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn new(func: fn(K) -> V, cache_capacity: NonZeroUsize) -> Self {
        Self {
            func,
            cache: LruCache::new(cache_capacity),
        }
    }

    fn call(&mut self, input: K) -> V {
        let cache = &mut self.cache;
        if let Some(output) = cache.get(&input) {
            return output.clone();
        }
        let output = (self.func)(input.clone());
        cache.put(input, output.clone());
        output
    }
}
