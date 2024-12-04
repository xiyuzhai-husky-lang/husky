// use berserk::{berserk, db::BerserkerDb, memo};

// #[berserk]
// pub struct Coword<'db> {
//     #[return_ref]
//     data: String,
// }

// impl<'db> std::fmt::Debug for Coword<'db> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("Coword").field(&self.data()).finish()
//     }
// }

// #[test]
// fn coword_works() {
//     let db = &BerserkerDb::default();
//     // Test creation and basic equality
//     let word1 = Coword::new("hello".to_string(), db);
//     let word2 = Coword::new("hello".to_string(), db);
//     let word3 = Coword::new("world".to_string(), db);

//     // Test equality for same content
//     assert_eq!(word1, word2);

//     // Test inequality for different content
//     assert_ne!(word1, word3);

//     // Test interning - should return same instance for same content
//     assert_eq!(word1, word2);
//     assert_ne!(word1, word3);

//     // Test access to underlying data
//     assert_eq!(word1.data(), "hello");
//     assert_eq!(word3.data(), "world");

//     // assert_eq!(db.interners.len(), 2);
// }

// #[memo]
// fn first_letter(word: Coword, db: &BerserkerDb) -> char {
//     word.data().chars().next().unwrap()
// }

// #[test]
// fn coword_first_letter_works() {
//     let db = &BerserkerDb::default();
//     let word = Coword::new("hello".to_string(), db);
//     assert_eq!(first_letter(word, db), 'h');
// }
