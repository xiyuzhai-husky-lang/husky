use atom::AtomContext;
use text::TextRange;
use vec_map::VecMapEntry;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDecl {
    pub liason: ParameterLiason,
    pub ty: EntityRoutePtr,
    pub ident: CustomIdentifier,
}

impl VecMapEntry<CustomIdentifier> for ParameterDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl ParameterDecl {
    pub fn from_static(symbol_context: &mut dyn AtomContext, input: &StaticParameter) -> Self {
        // opt_this_ty,
        Self {
            ty: symbol_context.parse_entity_route(input.ty).unwrap(),
            liason: input.contract,
            ident: symbol_context.entity_syntax_db().custom_ident(input.name),
        }
    }

    pub fn from_field(db: &dyn DeclQueryGroup, field_decl: &FieldDecl) -> InferResult<Self> {
        Ok(ParameterDecl {
            liason: ParameterLiason::from_member(
                field_decl.liason,
                field_decl.ty,
                db.is_copyable(field_decl.ty)?,
            ),
            ty: field_decl.ty,
            ident: field_decl.ident,
        })
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            ty: instantiator
                .instantiate_entity_route(self.ty)
                .take_entity_route(),
            liason: self.liason,
            ident: self.ident,
        }
    }

    pub fn implement(&self, implementor: &Implementor) -> Self {
        todo!()
    }
}

impl Into<ParameterDecl> for &Parameter {
    fn into(self) -> ParameterDecl {
        ParameterDecl {
            liason: self.liason,
            ty: self.ranged_ty.route,
            ident: self.ranged_ident.ident,
        }
    }
}
