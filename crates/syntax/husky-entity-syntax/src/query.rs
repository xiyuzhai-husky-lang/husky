use crate::*;
use husky_check_utils::should;
use husky_dev_utils::dev_src;
use husky_entity_kind::{EntityKind, MemberKind, TyKind};
use husky_entity_path::EntityPathItd;
use husky_path::PathItd;
use husky_path_utils::*;
use husky_print_utils::msg_once;
use husky_static_defn::*;
use husky_term::*;
use husky_word::{
    dash_to_snake, CustomIdentifier, Identifier, RootBuiltinIdentifier, Word, WordItd,
};
use thin_vec::{thin_vec, ThinVec};
use upcast::Upcast;

use fold::FoldableStorage;

use std::{path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntitySyntaxSalsaQueryGroup:
    husky_token_text::TokenizedTextQueryGroup + ResolveStaticRootDefn
{
    fn subroute_table(&self, entity_path: EntityPathItd) -> EntitySyntaxResultArc<SubrouteTable>;

    fn subentity_routes(&self, entity_path: EntityPathItd) -> Arc<Vec<Ty>>;
    fn subentity_kinded_routes(&self, entity_path: EntityPathItd) -> Arc<Vec<(EntityKind, Ty)>>;

    fn husky_entity_kind(&self, entity_path: EntityPathItd) -> EntitySyntaxResult<EntityKind>;

    fn entity_source(&self, entity_path: EntityPathItd) -> EntitySyntaxResult<EntitySource>;

    fn submodules(&self, module: Ty) -> Arc<Vec<Ty>>;
}

fn subroute_table(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_path: EntityPathItd,
) -> EntitySyntaxResultArc<SubrouteTable> {
    let husky_entity_kind = db.husky_entity_kind(entity_path)?;
    match db.husky_entity_kind(entity_path)? {
        EntityKind::Function { .. }
        | EntityKind::Feature
        | EntityKind::EnumVariant
        | EntityKind::Main
        | EntityKind::Member(_) => Ok(Arc::new(SubrouteTable::new(entity_path, husky_entity_kind))),
        EntityKind::Module | EntityKind::Type(_) | EntityKind::Trait => {
            Ok(Arc::new(match db.entity_source(entity_path)? {
                EntitySource::StaticModuleItem(data) => {
                    SubrouteTable::from_static(db, entity_path, husky_entity_kind, data)
                }
                EntitySource::WithinModule {
                    file,
                    token_group_index,
                } => {
                    let text = db.tokenized_text(file)?;
                    let item = text.iter_from(token_group_index).next().unwrap();
                    if let Some(children) = item.opt_children {
                        SubrouteTable::parse(db, file, entity_path, husky_entity_kind, children)
                    } else {
                        SubrouteTable::new(entity_path, husky_entity_kind)
                    }
                }
                EntitySource::Module { file: file_id } => {
                    let text = db.tokenized_text(file_id)?;
                    SubrouteTable::parse(db, file_id, entity_path, husky_entity_kind, text.iter())
                }
                EntitySource::WithinBuiltinModule => todo!(),
                EntitySource::TargetInput { .. } => todo!(),
                EntitySource::StaticTypeMember(_) => todo!(),
                EntitySource::StaticTraitMember(_) => todo!(),
                EntitySource::StaticTypeAsTraitMember => todo!(),
                EntitySource::Any { .. } => todo!(),
                EntitySource::StaticEnumVariant(_) => todo!(),
                EntitySource::ThisType { .. } => todo!(),
            }))
        }
    }
}

fn subentity_routes(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_path: EntityPathItd,
) -> Arc<Vec<Ty>> {
    Arc::new(db.subroute_table(entity_path).map_or(Vec::new(), |table| {
        todo!()
        // table.subroute_iter(db, entity_path).collect()
    }))
}

fn subentity_kinded_routes(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_path: EntityPathItd,
) -> Arc<Vec<(EntityKind, Ty)>> {
    Arc::new(db.subroute_table(entity_path).map_or(Vec::new(), |table| {
        todo!()
        // table.subentity_kinded_route_iter(db, entity_path).collect()
    }))
}

fn submodules(db: &dyn EntitySyntaxSalsaQueryGroup, module: Ty) -> Arc<Vec<Ty>> {
    todo!()
    // Arc::new(
    //     db.subroute_table(module)
    //         .unwrap()
    //         .submodule_route_iter(db, module)
    //         .collect(),
    // )
}

fn husky_entity_kind(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_path: EntityPathItd,
) -> EntitySyntaxResult<EntityKind> {
    todo!()
    // entity_kind_from_entity_route_kind(db, &entity_path.variant)
}

fn entity_source(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_path: EntityPathItd,
) -> EntitySyntaxResult<EntitySource> {
    todo!()
    // if entity_path.canonicalize().kind() != CanonicalTyKind::Intrinsic {
    //     panic!("expect intrinsic, but get `{}`", entity_path)
    // }
    // match entity_path.variant {
    //     EntityRouteVariant::Root { ident } => Ok(EntitySource::StaticModuleItem(db
    //         .__root_defn_resolver()(
    //         ident
    //     ))),
    //     EntityRouteVariant::Package { main, .. } => Ok(EntitySource::Module { file: main }),
    //     EntityRouteVariant::Child { parent, ident } => {
    //         db.subroute_table(parent)?.entity_source(ident)
    //     }
    //     EntityRouteVariant::TargetInputValue => Ok(EntitySource::TargetInput),
    //     EntityRouteVariant::Any {
    //         ident, file, range, ..
    //     } => Ok(EntitySource::Any {
    //         route: entity_path,
    //         ident,
    //         file,
    //         range,
    //     }),
    //     EntityRouteVariant::ThisType { file, range } => Ok(EntitySource::ThisType { file, range }),
    //     EntityRouteVariant::TypeAsTraitMember { ty, trai, ident: _ } => match trai {
    //         Ty::Root(root_ident) => match root_ident {
    //             RootBuiltinIdentifier::CloneTrait => {
    //                 msg_once!("ad hoc");
    //                 Ok(EntitySource::StaticTypeAsTraitMember)
    //                 // db.entity_source(EntityRoutePtr::Root(RootBuiltinIdentifier::CloneTrait))
    //             }
    //             _ => todo!(),
    //         },
    //         Ty::Custom(_) => match trai.variant {
    //             EntityRouteVariant::ThisType { file: _, range: _ } => {
    //                 let ty_source = db.entity_source(ty).unwrap();
    //                 match ty_source {
    //                     EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
    //                         EntityStaticDefnVariant::Ty { .. } => {
    //                             Ok(EntitySource::StaticTypeAsTraitMember)
    //                         }
    //                         _ => panic!(),
    //                     },
    //                     EntitySource::WithinBuiltinModule => todo!(),
    //                     EntitySource::WithinModule { .. } => todo!(),
    //                     EntitySource::Module { .. } => todo!(),
    //                     EntitySource::TargetInput { .. } => todo!(),
    //                     EntitySource::StaticTypeMember(_) => todo!(),
    //                     EntitySource::StaticTraitMember(_) => todo!(),
    //                     EntitySource::StaticTypeAsTraitMember => todo!(),
    //                     EntitySource::Any { .. } => todo!(),
    //                     EntitySource::StaticEnumVariant(_) => todo!(),
    //                     EntitySource::ThisType { .. } => todo!(),
    //                 }
    //             }
    //             _ => todo!(),
    //         },
    //     },
    //     EntityRouteVariant::TargetOutputType => todo!(),
    // }
}

pub struct ModuleFromFileError {
    pub rule_broken: ModuleFromFileRule,
}

pub enum ModuleFromFileRule {
    PackageNameShouldBeIdentifier,
    PackageRootShouldHaveFileName,
    FileShouldExist,
    FileShouldHaveExtensionHSK,
}

pub trait EntitySyntaxQueryGroup:
    EntitySyntaxSalsaQueryGroup + Upcast<dyn EntitySyntaxSalsaQueryGroup>
{
    fn all_modules(&self) -> Vec<Ty> {
        todo!()
        // self.all_target_entrances()
        //     .iter()
        //     .map(|id| self.collect_modules(*id))
        //     .flatten()
        //     .collect()
    }

    fn all_source_files(&self) -> Vec<PathItd>
    where
        Self: Sized,
    {
        self.all_target_entrances()
            .iter()
            .map(|file| self.collect_source_files(*file))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<EntityPathItd> {
        todo!()
        // self.all_modules().into_iter()
    }

    fn collect_modules(&self, file: PathItd) -> Vec<EntityPathItd> {
        if let Ok(module) = self.module(file) {
            let mut modules = vec![module];
            self.subroute_table(module).ok().map(|table| {
                modules.extend(
                    table
                        .submodule_idents()
                        .into_iter()
                        .filter_map(|ident| {
                            self.submodule_file(file, ident.ident)
                                .map_or(None, |id| Some(self.collect_modules(id)))
                        })
                        .flatten(),
                );
            });
            modules
        } else {
            vec![]
        }
    }

    fn collect_source_files(&self, target_entrance: PathItd) -> Vec<PathItd> {
        should!(target_entrance.ends_with("main.hsy"));
        collect_all_source_files(target_entrance.parent().unwrap().to_path_buf())
            .into_iter()
            .map(|path| self.intern_path(path))
            .collect()
    }

    fn module(&self, file: PathItd) -> EntitySyntaxResult<EntityPathItd> {
        let path: PathBuf = file.to_path_buf();
        if !self.file_exists(file) {
            Err(derived_error!(format!("file doesn't exist")))?
        } else if path_has_file_name(&path, "main.hsy") {
            if let Some(pack_name) = path_parent_file_name_str(&path) {
                let snake_name = dash_to_snake(&pack_name);
                if let WordItd::Identifier(Identifier::Custom(ident)) =
                    self.word_itr().intern(Word::new(snake_name))
                {
                    todo!()
                    // Ok(self.intern_entity_route(EntityRoute::package(file, ident)))
                } else {
                    Err(derived_error!(format!("pack name should be identifier")))?
                }
            } else {
                Err(derived_error!(format!("pack root should have filename")))?
            }
        } else if path_has_extension(&path, "hsy") {
            let parent = self.module(query_not_none!(
                self.parent_module_file(file),
                format!("cannot find parent")
            )?)?;
            let word = self.it_word(path.file_stem().unwrap().to_str().unwrap());
            match word {
                WordItd::Keyword(kw) => Err(derived_error!(format!(
                    "expect custom identifier for module name, but got keyword {} instead",
                    kw.as_str()
                ))),
                WordItd::Opr(word_opr) => Err(derived_error!(format!(
                    "expect custom identifier for module name, but got word operator {} instead",
                    word_opr.as_str()
                ))),
                WordItd::Identifier(ident) => match ident {
                    Identifier::Root(_) => todo!(),
                    Identifier::Custom(ident) => Ok(
                        todo!(),
                        //     self.intern_entity_route(EntityRoute {
                        //     variant: EntityRouteVariant::Child { parent, ident },
                        //     temporal_arguments: thin_vec![],
                        //     spatial_arguments: thin_vec![],
                        // })
                    ),
                    Identifier::Contextual(_) => todo!(),
                },
                WordItd::Decorator(_) => todo!(),
                WordItd::Pattern(_) => todo!(),
            }
        } else {
            Err(derived_error!(format!(
                "file (path: {:?}) should have extension .hsy",
                path.to_str()
            )))?
        }
    }

    fn module_file(&self, module: Ty) -> EntitySyntaxResult<PathItd> {
        todo!()
        // Ok(match self.entity_source(module)? {
        //     EntitySource::StaticModuleItem(_) => panic!(),
        //     EntitySource::WithinModule { file, .. } => file,
        //     EntitySource::Module { file } => file,
        //     EntitySource::WithinBuiltinModule => todo!(),
        //     EntitySource::TargetInput { .. } => todo!(),
        //     EntitySource::StaticTypeMember(_) => todo!(),
        //     EntitySource::StaticTraitMember(_) => todo!(),
        //     EntitySource::StaticTypeAsTraitMember => todo!(),
        //     EntitySource::Any { .. } => todo!(),
        //     EntitySource::StaticEnumVariant(_) => todo!(),
        //     EntitySource::ThisType { .. } => todo!(),
        // })
    }

    // fn entity_kind_from_entity_route_variant(
    //     &self,
    //     entity_route_variant: &EntityRouteVariant,
    // ) -> EntitySyntaxResult<EntityKind> {
    //     entity_kind_from_entity_route_kind(self.upcast(), entity_route_variant)
    // }
}
