use husky_coword::Coword;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexTextTokenData {
    Word(Coword),
    Dollar,
}
