use crate::*;

pub type HirTemplateArguments = smallvec::SmallVec<[HirTemplateArgument; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateArgument {
    /// `Vacant` is used to repr abstract types
    ///
    /// say a list of any element type
    ///
    /// It doesn't mean two elements in the list can be of different type
    ///
    /// It just means that the type is capable of representing any list,
    /// saving the need to recompile.
    ///
    /// It should be noted that phantom template parameter should only accept vacant parameter.
    Vacant,
    Type(HirType),
    Constant(HirConstant),
}
