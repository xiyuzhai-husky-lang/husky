use crate::*;
use husky_term::Ty;
use husky_text::RangedCustomIdentifier;
use husky_vm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyOpnKind {
    Binary {
        opr: BinaryPureClosedOpr,
        this: Ty,
    },
    Prefix(PrefixOpr),
    FunctionModelCall,
    FunctionRoutineCall,
    StructCall,
    NewVecFromList,
    RecordCall,
    Field {
        field_ident: RangedCustomIdentifier,
        field_binding: Binding,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        method_route: Ty,
        output_binding: Binding,
    },
    Index {
        element_binding: Binding,
    },
}
