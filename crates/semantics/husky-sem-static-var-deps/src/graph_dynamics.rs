mod initial_value;

use crate::{builder::SemStaticVarDepsBuilder, static_var_deps::SemStaticVarDeps, *};
use ::graph_dynamics::{
    context::{IsGraphDynamicsContext, IsGraphDynamicsScheme},
    cycle_group::CycleGroupMap,
};
use husky_entity_kind::MajorFormKind;
use husky_entity_path::{
    path::{assoc_item::AssocItemPath, major_item::MajorItemPath, ItemPath, ItemPathId},
    region::RegionPath,
};
use husky_eth_signature::signature::attr::AttrEthTemplate;
use husky_sem_item_path_deps::{
    helpers::graph_dynamics::{
        item_sem_item_path_cycle_group_itd, item_sem_item_path_full_deps_cropped,
        SemItemPathDepsCyclceGroupItd, SemItemPathDepsGraphDepsScheme,
    },
    item_path_deps::item_sem_item_path_deps,
};
use propagate::PropagationResult;
use propagate::PropagationResultRef;

pub struct SemStaticVarDepsGraphDynamicsScheme {}

impl IsGraphDynamicsScheme for SemStaticVarDepsGraphDynamicsScheme {
    type Value = SemStaticVarDeps;

    const MAX_ITERATION: usize = 1000;
}

#[derive(Clone, Copy)]
pub struct SemStaticVarDepsGraphDynamicsContext<'db> {
    db: &'db ::salsa::Db,
}

impl<'db> IsGraphDynamicsContext<'db> for SemStaticVarDepsGraphDynamicsContext<'db> {
    type DepsScheme = SemItemPathDepsGraphDepsScheme;

    type DynamicsScheme = SemStaticVarDepsGraphDynamicsScheme;

    fn deps_cropped(self, node: ItemPath) -> impl IntoIterator<Item = ItemPath> {
        item_sem_item_path_deps(self.db, *node)
            .as_ref()
            .unwrap()
            .iter()
            .copied()
    }

    fn full_deps_cropped(self, node: ItemPath) -> &'db [ItemPath] {
        item_sem_item_path_full_deps_cropped(self.db, *node)
    }

    fn cycle_group_itd(self, node: ItemPath) -> SemItemPathDepsCyclceGroupItd {
        item_sem_item_path_cycle_group_itd(self.db, *node)
    }

    fn initial_value(self, node: ItemPath) -> SemStaticVarDeps {
        item_sem_static_var_deps_initial_value(self.db, *node).clone()
    }

    fn updated_value<'a>(
        self,
        node: ItemPath,
        f: impl Fn(ItemPath) -> &'a SemStaticVarDeps,
    ) -> SemStaticVarDeps {
        let mut value = f(node).clone();
        let Some(mut builder) =
            SemStaticVarDepsBuilder::new(self.db, RegionPath::ItemDefn(node), f)
        else {
            return value;
        };
        value.merge(&builder.calc_root(), &mut Default::default());
        value
    }

    fn cycle_group_final_values(
        self,
        cycle_group_itd: SemItemPathDepsCyclceGroupItd,
    ) -> PropagationResultRef<
        'db,
        &'db ::graph_dynamics::cycle_group::CycleGroupMap<Self::DepsScheme, SemStaticVarDeps>,
    > {
        item_sem_static_var_deps_cycle_group_final_values(self.db, cycle_group_itd).as_ref()
    }
}

#[salsa::tracked(return_ref)]
fn item_sem_static_var_deps_initial_value(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> SemStaticVarDeps {
    use husky_entity_tree::node::attr::HasAttrPaths;

    let attr_paths = item_path_id.attr_paths(db);
    let mut deps = SemStaticVarDeps::default();
    let item_path = item_path_id.item_path(db);
    match item_path {
        ItemPath::Submodule(_, _) => (),
        ItemPath::MajorItem(major_item_path) => match major_item_path {
            MajorItemPath::Type(_) => (),
            MajorItemPath::Trait(_) => (),
            MajorItemPath::Form(major_form_path) => match major_form_path.kind(db) {
                MajorFormKind::Ritchie(_) => (),
                MajorFormKind::TypeAlias => (),
                MajorFormKind::TypeVar => (),
                MajorFormKind::Val => (),
                MajorFormKind::StaticVar => deps.insert(item_path),
                MajorFormKind::StaticMut => (),
                MajorFormKind::Compterm => (),
                MajorFormKind::Conceptual => (),
            },
        },
        ItemPath::AssocItem(assoc_item_path) => match assoc_item_path {
            AssocItemPath::TypeItem(_) => (),         // ad hoc
            AssocItemPath::TraitItem(_) => (),        // ad hoc
            AssocItemPath::TraitForTypeItem(_) => (), // ad hoc
        },
        ItemPath::TypeVariant(_, _) => (),
        ItemPath::ImplBlock(_) => (),
        ItemPath::Attr(_, _) => (),
        ItemPath::Chunk(_, _) => (),
    };
    for attr_path in attr_paths {
        use husky_eth_signature::signature::HasEthTemplate;

        let AttrEthTemplate::Deps(deps_eth_template) = attr_path.eth_template(db).unwrap() else {
            continue;
        };
        todo!()
    }
    deps
}

#[salsa::tracked(return_ref)]
fn item_sem_static_var_deps_cycle_group_final_values(
    db: &::salsa::Db,
    cycle_group_itd: SemItemPathDepsCyclceGroupItd,
) -> PropagationResult<CycleGroupMap<SemItemPathDepsGraphDepsScheme, SemStaticVarDeps>> {
    let ctx = SemStaticVarDepsGraphDynamicsContext { db };
    let cycle_group = cycle_group_itd.cycle_group(db);
    ctx.calc_cycle_group_final_values(cycle_group)
}

pub fn item_sem_static_var_deps<'db>(
    db: &'db ::salsa::Db,
    item_path_id: ItemPathId,
) -> &'db SemStaticVarDeps {
    let ctx = SemStaticVarDepsGraphDynamicsContext { db };
    ctx.final_value(item_path_id.item_path(db)).unwrap()
}

#[test]
fn item_sem_static_var_deps_works() {
    use husky_entity_tree::node::ItemSynNodePath;

    DB::ast_rich_test_debug_with_db(
        |db, item_syn_node_path: ItemSynNodePath| {
            item_syn_node_path
                .unambiguous_item_path(db)
                .map(|item_path| item_sem_static_var_deps(db, *item_path))
        },
        &AstTestConfig::new(
            "item_sem_static_var_deps",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    );
}
