mod error;

pub use self::error::*;

use husky_opn_syntax::Bracket;
use husky_token::*;
use husky_vfs::{ModulePath, VfsDb};
use husky_visibility::Visibility;
use parsec::ParseContext;

#[derive(Debug, PartialEq, Eq)]
pub struct VisibilityExpr {
    visibility: Visibility,
    variant: VisibilityExprVariant,
}

impl VisibilityExpr {
    #[inline(always)]
    pub fn new_protected(module_path: ModulePath) -> Self {
        VisibilityExpr {
            visibility: Visibility::PubUnder(module_path),
            variant: VisibilityExprVariant::Protected,
        }
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn parse_from_token_stream(
        db: &dyn VfsDb,
        module_path: ModulePath,
        token_stream: &mut TokenStream<'_>,
    ) -> VisibilityExprResult<Self> {
        Ok(if let Some(pub_token) = token_stream.parse::<PubToken>()? {
            if let Some(lpar) = token_stream.parse::<LeftParenthesisToken>()? {
                let state = token_stream.state();
                if let Some(crate_token) = token_stream.parse::<CrateToken>()? {
                    todo!()
                } else if let Some(super_token) = token_stream.parse::<SuperToken>()? {
                    VisibilityExpr {
                        visibility: Visibility::PubUnder(
                            module_path
                                .parent(db)
                                .ok_or(OriginalVisibilityExprError::NoSuperForRoot(state))?,
                        ),
                        variant: VisibilityExprVariant::PubUnder {
                            pub_token,
                            lpar,
                            scope: VisibilityScopeExpr::Super(super_token),
                            rpar: token_stream.parse_expected(
                                OriginalVisibilityExprError::ExpectRightParenthesis,
                            )?,
                        },
                    }
                } else {
                    Err(OriginalVisibilityExprError::ExpectCrateOrSuper(state))?
                }
            } else {
                VisibilityExpr {
                    visibility: Visibility::Pub,
                    variant: VisibilityExprVariant::Pub { pub_token },
                }
            }
        } else {
            Self::new_protected(module_path)
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VisibilityExprVariant {
    Protected,
    Pub {
        pub_token: PubToken,
    },
    PubUnder {
        pub_token: PubToken,
        lpar: LeftParenthesisToken,
        scope: VisibilityScopeExpr,
        rpar: RightParenthesisToken,
    },
}

/// it's guaranteed that the scope is appropriate
#[derive(Debug, PartialEq, Eq)]
pub enum VisibilityScopeExpr {
    Crate(CrateToken),
    Super(SuperToken),
}
