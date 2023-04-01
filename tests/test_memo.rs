use std::num::NonZeroUsize;
use std::time::Instant;

use lru::memo::create_memo;
use rand::Rng;

#[test]
fn test_memo() {
    let mut rng = rand::thread_rng();
    let numbers = (0..1000).map(|_| rng.gen_range(0..100)).collect::<Vec<_>>();

    let mut memo_nth_prime = create_memo(nth_prime, NonZeroUsize::new(100).unwrap());

    // test same results
    for n in &numbers {
        assert_eq!(memo_nth_prime(*n), nth_prime(*n));
    }

    // test better performance for memo
    let start = Instant::now();
    for n in &numbers {
        nth_prime(*n);
    }
    let std_time = start.elapsed();

    let start = Instant::now();
    for n in &numbers {
        memo_nth_prime(*n);
    }
    let memo_time = start.elapsed();

    assert!(memo_time < std_time);
    println!("Normal function took {std_time:?}");
    println!("Memo function took {memo_time:?}");
}

fn nth_prime(n: usize) -> usize {
    let mut i = 0;
    let mut prime = 2;
    let mut current = 2;
    while i < n {
        current += 1;
        if is_prime(current) {
            prime = current;
            i += 1;
        }
    }
    prime
}

fn is_prime(n: usize) -> bool {
    n > 1 && (2..n).filter(|x| n % x == 0).count() < 1
}

#[test]
fn test_nth_prime() {
    assert_eq!(2, nth_prime(0));
    assert_eq!(3, nth_prime(1));
    assert_eq!(5, nth_prime(2));
    assert_eq!(193, nth_prime(43));
    assert_eq!(643, nth_prime(116));
}

#[test]
fn test_is_prime() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(!is_prime(6));
    assert!(is_prime(7));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(is_prime(11));
    assert!(is_prime(13));
    assert!(is_prime(67));
    assert!(is_prime(719));
    assert!(is_prime(1061));
    assert!(!is_prime(1067));
}
