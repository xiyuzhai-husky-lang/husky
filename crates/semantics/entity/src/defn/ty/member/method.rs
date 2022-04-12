use avec::Avec;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct MethodDefn {
    pub ident: CustomIdentifier,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
    pub this_contract: InputContract,
    pub variant: MethodDefnVariant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MethodDefnVariant {
    Func { stmts: Avec<FuncStmt> },
    Proc { stmts: Avec<ProcStmt> },
    Pattern { stmts: Avec<LazyStmt> },
}

impl HasKey<CustomIdentifier> for MethodDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}
