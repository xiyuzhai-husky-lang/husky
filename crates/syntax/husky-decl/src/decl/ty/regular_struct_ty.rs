use super::*;

use husky_token::{CommaToken, LeftCurlyBraceToken, RightCurlyBraceToken};
use parsec::parse_separated_list2;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RegularStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub lcurl: LeftCurlyBraceToken,
    #[return_ref]
    field_comma_list: (Vec<FieldDeclPattern>, Vec<CommaToken>),
    #[return_ref]
    pub rcurl: RightCurlyBraceToken,
}

impl RegularStructTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }

    pub fn fields<'a>(self, db: &'a dyn DeclDb) -> &'a [FieldDeclPattern] {
        &self.field_comma_list(db).0
    }
}
impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_struct_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        children: DefnChildren,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse()?;
        if let Some(lcurl) = ctx.parse::<LeftCurlyBraceToken>()? {
            let (parameters, commas) = parse_separated_list2(&mut ctx, |e| e)?;
            let rcurl = ctx.parse_expected(OriginalDeclExprError::ExpectRightCurlyBrace)?;
            Ok(RegularStructTypeDecl::new(
                self.db(),
                path,
                ast_idx,
                parser.finish(),
                implicit_parameters,
                lcurl,
                (parameters, commas),
                rcurl,
            )
            .into())
        } else if let Some(_lbox) = ctx.parse::<LeftBoxBracketToken>()? {
            todo!()
        } else {
            Err(OriginalDeclError::ExpectLCurlOrLParOrSemicolon(ctx.save_state()).into())
        }
    }
}
