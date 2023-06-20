use super::*;
use husky_expr::ExprIdx;
use parsec::SeparatedListWithKet;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleStructTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub lpar: LeftParenthesisToken,
    #[return_ref]
    field_comma_list: SeparatedListWithKet<
        TupleStructFieldDeclPattern,
        CommaToken,
        RightParenthesisToken,
        DeclExprError,
    >,
    pub expr_region: ExprRegion,
}

impl TupleStructTypeNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::implicit_parameters)
        //     .unwrap_or(&[])
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [TupleStructFieldDeclPattern] {
        todo!()
        // &self.field_comma_list(db).0
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TupleStructTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub lpar: LeftParenthesisToken,
    #[return_ref]
    field_comma_list: (Vec<TupleStructFieldDeclPattern>, Vec<CommaToken>),
    pub rpar: RightParenthesisToken,
    pub expr_region: ExprRegion,
}

impl TupleStructTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: TupleStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [TupleStructFieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldDecl {
    ty: ExprIdx,
}
