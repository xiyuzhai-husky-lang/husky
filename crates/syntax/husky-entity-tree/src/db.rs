use crate::*;
use husky_check_utils::should;
use husky_dev_utils::dev_src;
use husky_entity_kind::{EntityKind, MemberKind, TyKind};
use husky_entity_path::EntityPath;
use husky_package_path::PackagePath;
use husky_path_utils::*;
use husky_print_utils::msg_once;
use husky_source_path::SourcePath;
use husky_term::*;
use husky_token::TokenDb;
use husky_word::Identifier;
use salsa::DbWithJar;
use thin_vec::{thin_vec, ThinVec};
use upcast::Upcast;

use std::{path::PathBuf, sync::Arc};

pub trait EntityTreeDb: DbWithJar<EntityTreeJar> + TokenDb {
    fn subroute_table(&self, entity_path: EntityPath) -> EntityTreeResultArc<SubrouteTable>;

    fn subentity_routes(&self, entity_path: EntityPath) -> Arc<Vec<Term>>;
    fn subentity_kinded_routes(&self, entity_path: EntityPath) -> Arc<Vec<(EntityKind, Term)>>;

    fn husky_entity_kind(&self, entity_path: EntityPath) -> EntityTreeResult<EntityKind>;

    fn submodules(&self, module: EntityPath) -> &[EntityPath];

    fn root_modules_of_package(&self, package: PackagePath) -> &[EntityPath];

    fn all_modules_of_package(&self, package: PackagePath) -> &[EntityPath];

    fn collect_source_files(&self, target_entrance: SourcePath) -> Vec<SourcePath> {
        todo!()
        // should!(target_entrance.ends_with("main.hsy"));
        // collect_all_source_files(target_entrance.parent().unwrap().to_path_buf())
        //     .into_iter()
        //     .map(|path| self.intern_path(path))
        //     .collect()
    }
}

impl<T> EntityTreeDb for T
where
    T: DbWithJar<EntityTreeJar> + TokenDb,
{
    fn subroute_table(&self, entity_path: EntityPath) -> EntityTreeResultArc<SubrouteTable> {
        todo!()
    }

    fn subentity_routes(&self, entity_path: EntityPath) -> Arc<Vec<Term>> {
        todo!()
    }

    fn subentity_kinded_routes(&self, entity_path: EntityPath) -> Arc<Vec<(EntityKind, Term)>> {
        todo!()
    }

    fn husky_entity_kind(&self, entity_path: EntityPath) -> EntityTreeResult<EntityKind> {
        todo!()
    }

    fn submodules(&self, module: EntityPath) -> &[EntityPath] {
        todo!()
    }

    fn root_modules_of_package(&self, package: PackagePath) -> &[EntityPath] {
        todo!()
    }

    fn all_modules_of_package(&self, package: PackagePath) -> &[EntityPath] {
        todo!()
    }
}

fn subroute_table(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResultArc<SubrouteTable> {
    todo!()
    // let husky_entity_kind = db.husky_entity_kind(entity_path)?;
    // match db.husky_entity_kind(entity_path)? {
    //     EntityKind::Function { .. }
    //     | EntityKind::Feature
    //     | EntityKind::EnumVariant
    //     | EntityKind::Main
    //     | EntityKind::Member(_) => Ok(Arc::new(SubrouteTable::new(entity_path, husky_entity_kind))),
    //     EntityKind::Module | EntityKind::Type(_) | EntityKind::Trait => {
    //         Ok(Arc::new(match db.entity_source(entity_path)? {
    //             EntitySource::StaticModuleItem(data) => {
    //                 SubrouteTable::from_static(db, entity_path, husky_entity_kind, data)
    //             }
    //             EntitySource::WithinModule {
    //                 file,
    //                 token_group_index,
    //             } => {
    //                 let text = db.tokenized_text(file)?;
    //                 let item = text.iter_from(token_group_index).next().unwrap();
    //                 if let Some(children) = item.opt_children {
    //                     SubrouteTable::parse(db, file, entity_path, husky_entity_kind, children)
    //                 } else {
    //                     SubrouteTable::new(entity_path, husky_entity_kind)
    //                 }
    //             }
    //             EntitySource::Module { file: file_id } => {
    //                 let text = db.tokenized_text(file_id)?;
    //                 SubrouteTable::parse(db, file_id, entity_path, husky_entity_kind, text.iter())
    //             }
    //             EntitySource::WithinBuiltinModule => todo!(),
    //             EntitySource::TargetInput { .. } => todo!(),
    //             EntitySource::StaticTypeMember(_) => todo!(),
    //             EntitySource::StaticTraitMember(_) => todo!(),
    //             EntitySource::StaticTypeAsTraitMember => todo!(),
    //             EntitySource::Any { .. } => todo!(),
    //             EntitySource::StaticEnumVariant(_) => todo!(),
    //             EntitySource::ThisType { .. } => todo!(),
    //         }))
    //     }
    // }
}

fn subentity_routes(db: &dyn EntityTreeDb, entity_path: EntityPath) -> Arc<Vec<Term>> {
    Arc::new(db.subroute_table(entity_path).map_or(Vec::new(), |table| {
        todo!()
        // table.subroute_iter(db, entity_path).collect()
    }))
}

fn subentity_kinded_routes(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> Arc<Vec<(EntityKind, Term)>> {
    Arc::new(db.subroute_table(entity_path).map_or(Vec::new(), |table| {
        todo!()
        // table.subentity_kinded_route_iter(db, entity_path).collect()
    }))
}

fn submodules(db: &dyn EntityTreeDb, module: Term) -> Arc<Vec<Term>> {
    todo!()
    // Arc::new(
    //     db.subroute_table(module)
    //         .unwrap()
    //         .submodule_route_iter(db, module)
    //         .collect(),
    // )
}

fn husky_entity_kind(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityKind> {
    todo!()
    // entity_kind_from_entity_route_kind(db, &entity_path.variant)
}

// fn entity_source(db: &dyn EntityTreeDb, entity_path: EntityPath) -> EntityTreeResult<EntitySource> {
//     todo!()
//     // if entity_path.canonicalize().kind() != CanonicalTyKind::Intrinsic {
//     //     panic!("expect intrinsic, but get `{}`", entity_path)
//     // }
//     // match entity_path.variant {
//     //     EntityRouteVariant::Root { ident } => Ok(EntitySource::StaticModuleItem(db
//     //         .__root_defn_resolver()(
//     //         ident
//     //     ))),
//     //     EntityRouteVariant::Package { main, .. } => Ok(EntitySource::Module { file: main }),
//     //     EntityRouteVariant::Child { parent, ident } => {
//     //         db.subroute_table(parent)?.entity_source(ident)
//     //     }
//     //     EntityRouteVariant::TargetInputValue => Ok(EntitySource::TargetInput),
//     //     EntityRouteVariant::Any {
//     //         ident, file, range, ..
//     //     } => Ok(EntitySource::Any {
//     //         route: entity_path,
//     //         ident,
//     //         file,
//     //         range,
//     //     }),
//     //     EntityRouteVariant::ThisType { file, range } => Ok(EntitySource::ThisType { file, range }),
//     //     EntityRouteVariant::TypeAsTraitMember { ty, trai, ident: _ } => match trai {
//     //         Term::Root(root_ident) => match root_ident {
//     //             RootBuiltinIdentifier::CloneTrait => {
//     //                 msg_once!("ad hoc");
//     //                 Ok(EntitySource::StaticTypeAsTraitMember)
//     //                 // db.entity_source(EntityRoutePtr::Root(RootBuiltinIdentifier::CloneTrait))
//     //             }
//     //             _ => todo!(),
//     //         },
//     //         Term::Custom(_) => match trai.variant {
//     //             EntityRouteVariant::ThisType { file: _, range: _ } => {
//     //                 let ty_source = db.entity_source(ty).unwrap();
//     //                 match ty_source {
//     //                     EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
//     //                         EntityStaticDefnVariant::Term { .. } => {
//     //                             Ok(EntitySource::StaticTypeAsTraitMember)
//     //                         }
//     //                         _ => panic!(),
//     //                     },
//     //                     EntitySource::WithinBuiltinModule => todo!(),
//     //                     EntitySource::WithinModule { .. } => todo!(),
//     //                     EntitySource::Module { .. } => todo!(),
//     //                     EntitySource::TargetInput { .. } => todo!(),
//     //                     EntitySource::StaticTypeMember(_) => todo!(),
//     //                     EntitySource::StaticTraitMember(_) => todo!(),
//     //                     EntitySource::StaticTypeAsTraitMember => todo!(),
//     //                     EntitySource::Any { .. } => todo!(),
//     //                     EntitySource::StaticEnumVariant(_) => todo!(),
//     //                     EntitySource::ThisType { .. } => todo!(),
//     //                 }
//     //             }
//     //             _ => todo!(),
//     //         },
//     //     },
//     //     EntityRouteVariant::TargetOutputType => todo!(),
//     // }
// }

pub struct ModuleFromFileError {
    pub rule_broken: ModuleFromFileRule,
}

pub enum ModuleFromFileRule {
    PackageNameShouldBeIdentifier,
    PackageRootShouldHaveFileName,
    FileShouldExist,
    FileShouldHaveExtensionHSK,
}
