mod field;
mod indirection;
mod method;

pub use self::field::*;
pub use self::indirection::*;
pub use self::method::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyMemberDisambiguation<S> {
    indirections: SmallVec<[FluffyIndirection; 2]>,
    ty_path: TypePath,
    signature: S,
}

impl<S> FluffyMemberDisambiguation<S> {
    fn merge(&self, mut indirections: SmallVec<[FluffyIndirection; 2]>) -> Self
    where
        S: Clone,
    {
        indirections.extend(self.indirections.iter().copied());
        Self {
            indirections,
            ty_path: self.ty_path,
            signature: self.signature.clone(),
        }
    }
}
