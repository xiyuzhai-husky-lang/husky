#[eterned::eterned]
pub struct Coword {
    pub data: String,
}

impl std::fmt::Debug for Coword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = eterned::db::attached_interner_db();
        f.debug_tuple("Coword").field(&self.data(db)).finish()
    }
}

impl std::fmt::Display for Coword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = eterned::db::attached_interner_db();
        write!(f, "{}", self.data(db))
    }
}

#[test]
fn coword_new_works() {
    let db = &eterned::db::EternerDb::default();

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
    assert_eq!(word1.data(db), "hello");
    assert_eq!(word3.data(db), "world");
}

#[test]
fn coword_from_ref_works() {
    let db = &eterned::db::EternerDb::default();

    // Test creation and basic equality
    let word1 = Coword::from_ref("hello", db);
    let word2 = Coword::from_ref("hello", db);
    let word3 = Coword::from_ref("world", db);

    // Test equality for same content
    assert_eq!(word1, word2);

    // Test inequality for different content
    assert_ne!(word1, word3);

    // Test interning - should return same instance for same content
    assert_eq!(word1, word2);
    assert_ne!(word1, word3);

    // Test access to underlying data
    assert_eq!(word1.data(db), "hello");
    assert_eq!(word3.data(db), "world");
}
