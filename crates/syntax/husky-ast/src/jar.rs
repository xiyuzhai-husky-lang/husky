#[salsa::jar]
pub struct AstJar(
    crate::sheet::ast_sheet,
    crate::range::ast_token_idx_range_sheet,
);
