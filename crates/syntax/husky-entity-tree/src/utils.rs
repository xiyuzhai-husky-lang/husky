use crate::*;
use helpers::paths::module_item_syn_node_paths;
use husky_entity_path::path::impl_block::TypeSketch;
use husky_path_utils::{Path, PathBuf};
use node::{assoc_item::AssocItemSynNodePath, impl_block::ImplBlockSynNodePath, ItemSynNodePathId};

impl ItemSynNodePath {
    pub(crate) fn stem(self, dir: &Path, test_name: &str, db: &::salsa::Db) -> PathBuf {
        let index = item_syn_node_path_stem_index(db, *self);
        let dir = self
            .module_path(db)
            .relative_stem(db)
            .to_logical_path(dir.join(test_name));
        if index == 0 {
            dir.join(self.stem_inner(db).to_string())
        } else {
            dir.join(format!("{}({index})", self.stem_inner(db)))
        }
    }

    fn stem_inner(self, db: &::salsa::Db) -> &str {
        let stem_inner = item_syn_node_path_stem_inner(db, *self);
        check_compatibility(stem_inner);
        stem_inner
    }
}

#[salsa::tracked(return_ref)]
fn item_syn_node_path_stem_inner(
    db: &::salsa::Db,
    item_syn_node_path_id: ItemSynNodePathId,
) -> String {
    let item_syn_node_path = item_syn_node_path_id.syn_node_path(db);
    match item_syn_node_path {
        ItemSynNodePath::Submodule(_, slf) => slf.ident(db).data(db).to_string(),
        ItemSynNodePath::MajorItem(slf) => slf.ident(db).data(db).to_string(),
        ItemSynNodePath::TypeVariant(_, slf) => {
            format!(
                "{}--{}",
                slf.parent(db).ident(db).data(db),
                slf.ident(db).data(db)
            )
        }
        ItemSynNodePath::ImplBlock(slf) => match slf {
            ImplBlockSynNodePath::TypeImplBlock(slf) => {
                format!("impl-{}", slf.ty_path(db).ident(db).data(db))
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(slf) => format!(
                "impl-{}-for-{}",
                slf.trai_path(db).ident(db).data(db),
                match slf.ty_sketch(db) {
                    TypeSketch::DeriveAny => "_",
                    TypeSketch::Path(ty_path) => ty_path.ident(db).data(db),
                }
            ),
            ImplBlockSynNodePath::IllFormedImplBlock(_) => todo!(),
        },
        ItemSynNodePath::AssocItem(slf) => match slf {
            AssocItemSynNodePath::TypeItem(slf) => {
                format!(
                    "{}--{}",
                    slf.ty_path(db).ident(db).data(db),
                    slf.ident(db).data(db)
                )
            }
            AssocItemSynNodePath::TraitItem(slf) => {
                format!(
                    "{}--{}",
                    slf.parent_trai_syn_node_path(db).ident(db).data(db),
                    slf.ident(db).data(db)
                )
            }
            AssocItemSynNodePath::TraitForTypeItem(slf) => {
                format!(
                    "({}-as-{})--{}",
                    match slf.ty_sketch(db) {
                        TypeSketch::DeriveAny => "_",
                        TypeSketch::Path(ty_path) => ty_path.ident(db).data(db),
                    },
                    slf.trai_path(db).ident(db).data(db),
                    slf.ident(db).data(db)
                )
            }

            AssocItemSynNodePath::IllFormedItem(_) => todo!(),
        },
        ItemSynNodePath::Attr(_, slf) => {
            format!(
                "{}--#{}",
                item_syn_node_path_stem_inner(db, *slf.parent_syn_node_path(db)),
                slf.ident(db).data(db)
            )
        }
        ItemSynNodePath::Chunk(_, _) => todo!(),
    }
}

fn check_compatibility(s: &str) {
    assert!(!s.contains(":"));
    assert!(!s.contains("\""));
}

#[salsa::tracked]
fn item_syn_node_path_stem_index(
    db: &::salsa::Db,
    item_syn_node_path_id: ItemSynNodePathId,
) -> usize {
    let stem_inner = item_syn_node_path_stem_inner(db, item_syn_node_path_id);
    let module_path = item_syn_node_path_id.module_path(db);
    module_item_syn_node_paths(db, module_path)
        .iter()
        .filter(|&path| path.stem_inner(db) == stem_inner)
        .position(|&path| *path == item_syn_node_path_id)
        .unwrap()
}
