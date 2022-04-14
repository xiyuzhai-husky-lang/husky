use avec::Avec;
use entity_route::EntityRouteKind;
use print_utils::p;

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

pub(crate) fn method_defn(db: &dyn EntityDefnQueryGroup, route: EntityRoutePtr) -> Arc<MethodDefn> {
    let (parent, method_ident) = match route.kind {
        EntityRouteKind::ChildScope { parent, ident } => (parent, ident),
        _ => {
            p!(route.kind);
            panic!("")
        }
    };
    let ty_defn = db.entity_defn(parent).unwrap();
    let method = ty_defn.method(method_ident);
    todo!()
}
