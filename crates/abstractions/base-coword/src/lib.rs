#[cfg(feature = "pyo3")]
mod pyo3;
#[cfg(feature = "serde")]
mod serde;

#[eterned::eterned]
pub struct BaseCoword {
    #[return_ref(str)]
    pub data: String,
}

impl std::fmt::Debug for BaseCoword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Coword").field(&self.data()).finish()
    }
}

impl std::fmt::Display for BaseCoword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data())
    }
}

#[test]
fn coword_new_works() {
    let db = &eterned::db::EternerDb::default();

    // Test creation and basic equality
    let word1 = BaseCoword::new("hello".to_string(), db);
    let word2 = BaseCoword::new("hello".to_string(), db);
    let word3 = BaseCoword::new("world".to_string(), db);

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
}

#[test]
fn coword_from_ref_works() {
    let db = &eterned::db::EternerDb::default();

    // Test creation and basic equality
    let word1 = BaseCoword::from_ref("hello", db);
    let word2 = BaseCoword::from_ref("hello", db);
    let word3 = BaseCoword::from_ref("world", db);

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
}
