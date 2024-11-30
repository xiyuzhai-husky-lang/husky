use interned::{interned, memo};

#[interned]
pub struct Coword {
    data: String,
}

impl std::fmt::Debug for Coword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let db = interned::db::attached_interner_db();
        f.debug_tuple("Coword").field(self.data(db)).finish()
    }
}

#[test]
fn coword_works() {
    let db = interned::db::attached_interner_db();
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

    // assert_eq!(db.interners.len(), 2);
}

#[memo]
fn first_letter(word: Coword, db: &InternerDb) -> char {
    word.data(db).chars().next().unwrap()
}
