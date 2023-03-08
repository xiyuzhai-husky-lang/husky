use super::*;

pub struct CustomTypePath(TypePath);

impl From<CustomTypePath> for TypePath {
    fn from(path: CustomTypePath) -> Self {
        path.0
    }
}

impl TypePath {
    pub fn refine(
        self,
        db: &dyn EntityPathDb,
    ) -> EntityPathResult<Either<CustomTypePath, PreludeTypePath>> {
        Ok(match self.prelude_ty_path(db)? {
            None => Left(CustomTypePath(self)),
            Some(path) => Right(path),
        })
    }
}
