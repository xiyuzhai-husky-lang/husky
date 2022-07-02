use super::*;
use husky_entity_route_syntax::EntityRoutePtr;

/// GenericTypeCallLinkage
#[derive(Clone, Copy)]
pub struct GenericRoutineLinkage {
    pub call: for<'temp, 'eval> fn(
        ty: EntityRoutePtr,
        &mut [TempValue<'temp, 'eval>],
    ) -> EvalResult<TempValue<'temp, 'eval>>,
    pub nargs: u8,
    pub dev_src: &'static __StaticDevSource,
}

#[macro_export]
macro_rules! generic_routine_linkage {
    ($fp: expr, $nargs: expr) => {{
        GenericRoutineLinkage {
            call: $fp,
            nargs: $nargs,
            dev_src: &dev_utils::__static_dev_src!(),
        }
    }};
}

impl std::fmt::Debug for GenericRoutineLinkage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("GenericTypeCallFp(")?;
        (self.call as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl std::hash::Hash for GenericRoutineLinkage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.call as usize).hash(state);
    }
}

impl PartialEq for GenericRoutineLinkage {
    fn eq(&self, other: &Self) -> bool {
        (self.call as usize) == (other.call as usize)
    }
}

impl Eq for GenericRoutineLinkage {}
