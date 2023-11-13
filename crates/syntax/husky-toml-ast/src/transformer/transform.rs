use super::*;

impl<Context> TransformFromTomlAst<Context> for DiffPath
where
    Context: TomlDeserializeContext,
{
    type Ast = TomlExpr;

    #[inline(always)]
    fn transform_from<'a, 'b>(
        transformer: TomlTransformer<'a, 'b, Context, Self::Ast>,
    ) -> Result<Self, <Context as TomlDeserializeContext>::Error> {
        match transformer.visitor.expr() {
            TomlExpr::String(s) => {
                match DiffPath::try_new(
                    <Context::Db<'a> as salsa::DbWithJar<VfsJar>>::as_jar_db(transformer.db),
                    transformer.path.join(s as &str),
                ) {
                    Ok(path) => Ok(path),
                    Err(_) => todo!(),
                }
            }
            TomlExpr::Integer(_) => todo!(),
            TomlExpr::Float(_) => todo!(),
            TomlExpr::Boolean(_) => todo!(),
            TomlExpr::Datetime(_) => todo!(),
            TomlExpr::Array(_) => todo!(),
            TomlExpr::Table(_) => todo!(),
            TomlExpr::Err(_) => todo!(),
        }
    }
}
