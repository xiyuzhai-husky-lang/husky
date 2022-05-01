use entity_route::EntityRoutePtr;
use file::FilePtr;
use text::TextRange;

use crate::*;

#[derive(Debug, Clone)]
pub struct MutationData<'eval> {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: EntityRoutePtr,
    pub before: StackValueSnapshot<'eval>,
    pub after: StackValueSnapshot<'eval>,
}
