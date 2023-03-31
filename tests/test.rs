use std::num::NonZeroUsize;

use lru::cache::LruCache;

#[test]
fn test_cache() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = LruCache::<String, usize>::new(NonZeroUsize::try_from(5)?);
    assert_eq!(5, Into::<usize>::into(cache.capacity()));
    assert_eq!(0, cache.size());
    assert!(cache.get("apple").is_none());

    cache.put("apple".to_owned(), 2);
    cache.put("banana".to_owned(), 4);
    cache.put("mango".to_owned(), 1);
    cache.put("kiwi".to_owned(), 5);
    assert_eq!(4, cache.size());
    assert_eq!(1, *cache.get("mango").unwrap());
    assert_eq!(2, *cache.get("apple").unwrap());
    assert_eq!(5, *cache.get("kiwi").unwrap());
    assert_eq!(4, *cache.get("banana").unwrap());

    cache.put("orange".to_owned(), 3);
    cache.put("pineapple".to_owned(), 7);
    assert_eq!(5, cache.size());
    assert!(cache.get("mango").is_none());
    assert_eq!(3, *cache.get("orange").unwrap());
    assert_eq!(7, *cache.get("pineapple").unwrap());
    assert_eq!(2, *cache.get("apple").unwrap());
    assert_eq!(5, *cache.get("kiwi").unwrap());
    assert_eq!(4, *cache.get("banana").unwrap());
    assert_eq!(7, *cache.get("pineapple").unwrap());
    assert_eq!(7, *cache.get("pineapple").unwrap());
    assert_eq!(2, *cache.get("apple").unwrap());
    assert_eq!(5, *cache.get("kiwi").unwrap());

    cache.put("apple".to_owned(), 1);
    cache.put("banana".to_owned(), 8);
    cache.put("mango".to_owned(), 4);
    cache.put("kiwi".to_owned(), 3);
    assert_eq!(5, cache.size());
    assert!(cache.get("orange").is_none());
    assert_eq!(1, *cache.get("apple").unwrap());
    assert_eq!(8, *cache.get("banana").unwrap());
    assert_eq!(4, *cache.get("mango").unwrap());
    assert_eq!(3, *cache.get("kiwi").unwrap());
    assert_eq!(7, *cache.get("pineapple").unwrap());
    Ok(())
}
