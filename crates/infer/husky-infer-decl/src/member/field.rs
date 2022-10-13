use crate::*;
use husky_atom::AtomContext;
use husky_entity_kind::FieldKind;
use husky_entity_route::EntityRoutePtr;
use husky_instantiate::InstantiationContext;
use husky_word::CustomIdentifier;
use vec_like::VecMapEntry;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldDecl {
    pub ident: CustomIdentifier,
    pub modifier: MemberModifier,
    pub ty: EntityRoutePtr,
    pub field_kind: FieldKind,
}

impl FieldDecl {
    pub fn instantiate(&self, ctx: &InstantiationContext) -> Arc<Self> {
        Arc::new(Self {
            ident: self.ident,
            modifier: self.modifier,
            ty: self.ty.instantiate(ctx).take_entity_route(),
            field_kind: self.field_kind,
        })
    }

    pub fn from_static(
        symbol_context: &mut dyn AtomContext,
        static_decl: &EntityStaticDefn,
    ) -> Arc<Self> {
        match static_decl.variant {
            EntityStaticDefnVariant::TyField {
                field_kind,
                liason,
                field_ty,
                ..
            } => Arc::new(Self {
                ident: symbol_context
                    .entity_syntax_db()
                    .it_word(static_decl.name)
                    .custom(),
                modifier: liason,
                ty: symbol_context.parse_entity_route(field_ty).unwrap(),
                field_kind,
            }),
            _ => panic!(""),
        }
    }

    pub fn from_ast(ast: &Ast) -> Arc<Self> {
        match ast.variant {
            AstVariant::FieldDefnHead {
                liason,
                ranged_ident,
                field_ty: ty,
                ast_field_kind: field_ast_kind,
            } => Arc::new(Self {
                ident: ranged_ident.ident,
                modifier: liason,
                ty: ty.route,
                field_kind: field_ast_kind.into(),
            }),
            _ => panic!(),
        }
    }
}

impl VecMapEntry<CustomIdentifier> for FieldDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
