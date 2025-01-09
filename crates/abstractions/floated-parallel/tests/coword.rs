use floated_parallel::{db::FloaterDb, *};

#[floated]
pub struct Coword<'db> {
    #[return_ref]
    data: String,
}

impl std::fmt::Debug for Coword<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Coword").field(&self.data()).finish()
    }
}

#[test]
fn coword_works() {
    let db = &FloaterDb::default();
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

#[note]
fn first_letter<'db>(word: Coword<'db>, db: &'db FloaterDb) -> char {
    word.data().chars().next().unwrap()
}
