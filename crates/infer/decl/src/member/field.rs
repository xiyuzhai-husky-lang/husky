use crate::*;
use entity_route::EntityRoutePtr;
use instantiate::Instantiator;
use vec_map::HasKey;
use vm::MemberLiason;
use word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldDecl {
    pub ident: CustomIdentifier,
    pub liason: MemberLiason,
    pub ty: EntityRoutePtr,
    pub field_kind: FieldKind,
}

impl FieldDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        todo!()
    }

    pub fn from_static(db: &dyn DeclQueryGroup, static_decl: &EntityStaticDefn) -> Self {
        match static_decl.variant {
            EntityStaticDefnVariant::TypeField { ref field_variant } => Self {
                ident: db.intern_word(static_decl.name).custom(),
                liason: todo!(),
                ty: todo!(),
                field_kind: match *field_variant {},
            },
            _ => panic!(""),
        }
    }

    pub fn from_ast(ast: &Ast) -> Arc<Self> {
        match ast.variant {
            AstVariant::FieldDefnHead {
                liason,
                ranged_ident,
                ty,
                field_kind,
            } => Arc::new(Self {
                ident: ranged_ident.ident,
                liason,
                ty: ty.route,
                field_kind,
            }),
            _ => panic!(),
        }
    }
}

impl HasKey<CustomIdentifier> for FieldDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
