use crate::*;
use husky_entity_kind::FieldKind;
use husky_term::Ty;
use husky_text::RangedCustomIdentifier;
use husky_vm::Binding;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnVariant {
    Binary {
        opr: BinaryOpr,
    },
    Prefix {
        opr: PrefixOpr,
    },
    Suffix {
        opr: EagerSuffixOpr,
    },
    ValueCall,
    TypeCall,
    NewVecFromList,
    Field {
        this_ty: Ty,
        field_ident: RangedCustomIdentifier,
        field_liason: MemberModifier,
        field_binding: Binding,
        field_kind: FieldKind,
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
