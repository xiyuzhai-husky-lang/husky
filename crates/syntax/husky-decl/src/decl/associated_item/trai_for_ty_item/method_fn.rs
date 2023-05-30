use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeMethodFnDecl {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl TraitForTypeMethodFnDecl {
    pub fn self_parameter<'a>(self, db: &'a dyn DeclDb) -> Option<&'a SelfParameterDeclPattern> {
        self.explicit_parameter_decl_list(db)
            .self_parameter()
            .as_ref()
    }

    pub fn regular_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.explicit_parameter_decl_list(db).regular_parameters()
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}

impl<'a> DeclParseContext<'a> {}
