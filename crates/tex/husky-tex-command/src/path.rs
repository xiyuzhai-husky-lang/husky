#[salsa::interned]
pub struct TexCommandPath {
    data: TexCommandPathData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TexCommandPathData {
    Name(String),
}
