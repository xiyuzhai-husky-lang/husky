use ast::*;
use semantics_error::SemanticResult;
use syntax_types::{MembVarSignature, RawEnumVariantKind, RawTyKind};
use vec_map::VecMap;
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
}

impl Ty {
    pub(crate) fn from_ast(head: &Ast, children: AstIter) -> SemanticResult<Ty> {
        Ok(Ty {
            kind: match head.kind {
                AstKind::TypeDef {
                    ident,
                    kind,
                    ref generics,
                } => match kind {
                    RawTyKind::Enum => Self::enum_from_ast(children)?,
                    RawTyKind::Struct => Self::struct_from_ast(children)?,
                },
                _ => panic!(),
            },
        })
    }

    fn enum_from_ast(children: AstIter) -> SemanticResult<TyKind> {
        let mut variants = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::EnumVariant {
                    ident,
                    raw_variant_kind,
                } => {
                    let variant_kind = match raw_variant_kind {
                        RawEnumVariantKind::Constant => EnumVariantKind::Constant,
                    };
                    variants.insert_new(ident, variant_kind);
                }
                _ => panic!(),
            }
        }
        Ok(TyKind::Enum { variants })
    }

    fn struct_from_ast(children: AstIter) -> SemanticResult<TyKind> {
        let mut memb_vars = VecMap::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::Use { ident, scope } => (),
                AstKind::RoutineDecl {
                    ref routine_kind,
                    ref routine_head,
                } => todo!(),
                AstKind::MembVar { ident, signature } => memb_vars.insert_new(ident, signature),
                AstKind::MembRoutineDecl(_) => todo!(),
                _ => panic!(),
            }
        }
        Ok(TyKind::Struct { memb_vars })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyKind {
    Enum {
        variants: VecMap<CustomIdentifier, EnumVariantKind>,
    },
    Struct {
        memb_vars: VecMap<CustomIdentifier, MembVarSignature>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantKind {
    Constant,
}
