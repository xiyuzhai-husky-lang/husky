use crate::*;

use husky_entity_kind::{TraitItemKind, TypeItemKind};
use husky_entity_syn_tree::helpers::paths::{module_item_paths, module_item_syn_node_paths};
use husky_syn_expr::helpers::block_expr::parse_defn_block_expr;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemSynNodeDefn {
    pub body: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(jar = SynDefnJar)]
pub fn item_syn_node_defn(
    db: &::salsa::Db,
    item_syn_node_path_id: ItemSynNodePathId,
) -> Option<ItemSynNodeDefn> {
    let syn_node_path = item_syn_node_path_id.syn_node_path(db);
    let (allow_self_ty, allow_self_value) = match syn_node_path {
        ItemSynNodePath::MajorItem(MajorItemSynNodePath::Fugitive(_path)) => {
            (AllowSelfType::False, AllowSelfValue::False)
        }
        ItemSynNodePath::AssociatedItem(path) => (
            AllowSelfType::True,
            match path {
                AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                    match syn_node_path.data(db).item_kind(db) {
                        TypeItemKind::MethodFn => AllowSelfValue::True,
                        TypeItemKind::AssociatedFunctionFn => AllowSelfValue::False,
                        TypeItemKind::AssociatedVal => AllowSelfValue::True,
                        TypeItemKind::AssociatedType => AllowSelfValue::False,
                        TypeItemKind::MemoizedField => AllowSelfValue::True,
                    }
                }
                AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                    match syn_node_path.data(db).item_kind(db) {
                        TraitItemKind::MethodFn => AllowSelfValue::True,
                        TraitItemKind::AssociatedType => AllowSelfValue::False,
                        TraitItemKind::AssociatedVal => AllowSelfValue::True,
                        TraitItemKind::AssociatedFunctionFn => AllowSelfValue::False,
                    }
                }
                AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                    match syn_node_path.data(db).item_kind(db) {
                        TraitItemKind::MethodFn => AllowSelfValue::True,
                        TraitItemKind::AssociatedType => AllowSelfValue::False,
                        TraitItemKind::AssociatedVal => AllowSelfValue::True,
                        TraitItemKind::AssociatedFunctionFn => AllowSelfValue::False,
                    }
                }
                AssociatedItemSynNodePath::IllFormedItem(_) => todo!(),
            },
        ),
        _ => return None,
    };
    let (body, syn_expr_region) = parse_defn_block_expr(
        syn_node_path,
        syn_node_path.syn_node_decl(db).syn_expr_region(db)?,
        allow_self_ty,
        allow_self_value,
        db,
    )?;
    Some(ItemSynNodeDefn {
        body,
        syn_expr_region,
    })
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemSynDefn {
    pub body: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

pub fn item_syn_defn(db: &::salsa::Db, item_path: ItemPath) -> Option<ItemSynDefn> {
    let ItemSynNodeDefn {
        body,
        syn_expr_region,
    } = item_syn_node_defn(db, *item_path.syn_node_path(db))?;
    Some(ItemSynDefn {
        body,
        syn_expr_region,
    })
}

pub fn module_item_syn_node_defns(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(ItemSynNodePath, Option<ItemSynNodeDefn>)> {
    module_item_syn_node_paths(db, module_path)
        .iter()
        .copied()
        .map(|syn_node_path| (syn_node_path, item_syn_node_defn(db, *syn_node_path)))
        .collect()
}

#[test]
fn module_item_syn_node_defns_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_item_syn_node_defns(db, module_path),
        &AstTestConfig::new("module_item_syn_node_defns"),
    );
}

pub fn module_item_syn_defns(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(ItemPath, Option<ItemSynDefn>)> {
    module_item_paths(db, module_path)
        .iter()
        .copied()
        .map(|path| (path, item_syn_defn(db, path)))
        .collect()
}

#[test]
fn module_item_syn_defns_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_item_syn_defns(db, module_path),
        &AstTestConfig::new("module_item_syn_defns"),
    );
}
