use interned::interned;

#[interned]
pub struct Coword {
    data: String,
}

#[test]
fn coword_works() {
    // Test creation and basic equality
    let word1 = Coword::new("hello".to_string());
    let word2 = Coword::new("hello".to_string());
    let word3 = Coword::new("world".to_string());

    // Test equality for same content
    assert_eq!(word1, word2);

    // Test inequality for different content
    assert_ne!(word1, word3);

    // Test interning - should return same instance for same content
    assert_eq!(word1, word2);
    assert_ne!(word1, word3);

    // Test access to underlying data
    assert_eq!(word1.data(), "hello");
    assert_eq!(word3.data(), "world");

    assert_eq!(__COWORD_STORAGE.lock().unwrap().len_checked(), 2);
}
