use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
pub struct CustomTypePath(TypePath);

impl From<CustomTypePath> for TypePath {
    fn from(path: CustomTypePath) -> Self {
        path.0
    }
}

impl TypePath {
    pub fn refine(self, db: &::salsa::Db) -> Either<PreludeTypePath, CustomTypePath> {
        match self.prelude_ty_path(db) {
            Some(path) => Left(path),
            None => Right(CustomTypePath(self)),
        }
    }
}
