mod error;

pub use self::error::*;

use husky_scope::Scope;
use husky_token::*;
use husky_vfs::path::module_path::ModulePath;
use parsec::IsStreamParser;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VisibilityExpr {
    data: VisibilityExprData,
    visibility: Scope,
}

impl VisibilityExpr {
    #[inline(always)]
    pub fn new_protected(module_path: ModulePath) -> Self {
        VisibilityExpr {
            visibility: Scope::PubUnder(module_path),
            data: VisibilityExprData::Protected,
        }
    }

    pub fn visibility(&self) -> Scope {
        self.visibility
    }

    pub fn parse_from_token_stream(
        db: &::salsa::Db,
        module_path: ModulePath,
        token_stream: &mut TokenStream<'_>,
    ) -> VisibilityExprResult<Self> {
        Ok(
            if let Some(pub_token) = token_stream.try_parse_option::<PubToken>()? {
                if let Some(lpar) = token_stream.try_parse_option::<LparToken>()? {
                    let path_name_token: PathNameToken = token_stream
                        .try_parse_expected(OriginalVisibilityExprError::ExpectedCrateOrSuper)?;
                    match path_name_token {
                        PathNameToken::Ident(_) => todo!(),
                        PathNameToken::CrateRoot(_) => todo!(),
                        PathNameToken::SelfMod(_) => todo!(),
                        PathNameToken::Super(super_token) => VisibilityExpr {
                            visibility: Scope::PubUnder(module_path.parent(db).ok_or(
                                OriginalVisibilityExprError::NoSuperForRoot(
                                    super_token.token_idx(),
                                ),
                            )?),
                            data: VisibilityExprData::PubUnder {
                                pub_token,
                                lpar,
                                visibility: VisibilityScopeExpr::Super(super_token),
                                rpar: token_stream.try_parse_expected(
                                    OriginalVisibilityExprError::ExpectedRightParenthesis,
                                )?,
                            },
                        },
                    }
                } else {
                    VisibilityExpr {
                        visibility: Scope::Pub,
                        data: VisibilityExprData::Pub { pub_token },
                    }
                }
            } else {
                Self::new_protected(module_path)
            },
        )
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VisibilityExprData {
    Protected,
    Pub {
        pub_token: PubToken,
    },
    PubUnder {
        pub_token: PubToken,
        lpar: LparToken,
        visibility: VisibilityScopeExpr,
        rpar: RparToken,
    },
}

/// it's guaranteed that the visibility is appropriate
#[derive(Debug, PartialEq, Eq)]
pub enum VisibilityScopeExpr {
    Crate(CrateToken),
    Super(SuperToken),
}
