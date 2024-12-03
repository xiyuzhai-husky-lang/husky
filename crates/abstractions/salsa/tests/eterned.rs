use base_coword::Coword;
use std::cell::Cell;
use std::thread_local;

thread_local! {
    static CALL_COUNT: Cell<usize> = Cell::new(0);
}

#[salsa::jar]
struct Jar(first_letter);

#[salsa::tracked]
fn first_letter(db: &salsa::Db, word: Coword) -> char {
    CALL_COUNT.with(|count| count.set(count.get() + 1));
    word.data().chars().next().unwrap()
}

#[salsa::db(Jar)]
#[derive(Default)]
struct DB;

#[test]
fn test_first_letter() {
    let db = &DB::default();

    // First call
    let word = Coword::new("hello".to_string(), db);
    assert_eq!(first_letter(db, word), 'h');
    assert_eq!(CALL_COUNT.with(|c| c.get()), 1);

    // Same input should not trigger recomputation
    assert_eq!(first_letter(db, word), 'h');
    assert_eq!(CALL_COUNT.with(|c| c.get()), 1);

    // Different input should trigger recomputation
    let word2 = Coword::new("world".to_string(), db);
    assert_eq!(first_letter(db, word2), 'w');
    assert_eq!(CALL_COUNT.with(|c| c.get()), 2);

    // Test with uppercase
    let word3 = Coword::new("Test".to_string(), db);
    assert_eq!(first_letter(db, word3), 'T');
    assert_eq!(CALL_COUNT.with(|c| c.get()), 3);

    // Test with non-alphabetic characters
    let word4 = Coword::new("123abc".to_string(), db);
    assert_eq!(first_letter(db, word4), '1');
    assert_eq!(CALL_COUNT.with(|c| c.get()), 4);

    // Test with Unicode characters
    let word6 = Coword::new("über".to_string(), db);
    assert_eq!(first_letter(db, word6), 'ü');
    assert_eq!(CALL_COUNT.with(|c| c.get()), 5);
}
