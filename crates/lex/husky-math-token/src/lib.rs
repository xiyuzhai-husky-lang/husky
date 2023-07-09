mod error;
mod formula;
mod pos;

pub use self::error::*;
pub use self::formula::*;
pub use self::pos::*;

#[salsa::jar(db = MathTokenDb)]
pub struct MathTokenJar();

pub trait MathTokenDb: salsa::DbWithJar<MathTokenJar> {}

impl<Db> MathTokenDb for Db where Db: salsa::DbWithJar<MathTokenJar> {}
