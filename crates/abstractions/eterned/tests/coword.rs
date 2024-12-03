use eterned::{db::EternerDb, eterned, memo};

#[eterned]
pub struct Coword {
    #[return_ref]
    data: String,
}

impl std::fmt::Debug for Coword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Coword").field(&self.data()).finish()
    }
}

#[test]
fn coword_works() {
    let db = &EternerDb::default();
    // Test creation and basic equality
    let word1 = Coword::new("hello".to_string(), db);
    let word2 = Coword::new("hello".to_string(), db);
    let word3 = Coword::new("world".to_string(), db);

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

    // assert_eq!(db.interners.len(), 2);
}

#[memo]
fn first_letter(word: Coword, db: &EternerDb) -> char {
    word.data().chars().next().unwrap()
}
