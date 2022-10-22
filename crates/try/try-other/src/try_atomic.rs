use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn test_atomic() {
    // use static, don't use const here
    static NEXT_RAW: AtomicUsize = AtomicUsize::new(0);
    assert_eq!(NEXT_RAW.fetch_add(1, Ordering::SeqCst), 0);
    p!(NEXT_RAW.load(Ordering::SeqCst));
    assert_eq!(NEXT_RAW.fetch_add(1, Ordering::SeqCst), 1);
}
