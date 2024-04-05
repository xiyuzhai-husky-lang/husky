use husky_coword::Coword;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexCommandPath {
    Name(Coword),
}
