use husky_coword::Coword;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LxCommandPath {
    Coword(Coword),
}
