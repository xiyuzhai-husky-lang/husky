#[salsa::jar]
pub struct NamAstJar(
    crate::data::syntax::NamSyntaxAst,
    crate::data::notion::NamNotionAst,
    crate::data::semantics::NamSemanticsAst,
    crate::data::statement::NamStatementAst,
);
