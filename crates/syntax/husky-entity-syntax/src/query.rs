use crate::*;
use husky_check_utils::should;
use husky_dev_utils::dev_src;
use husky_entity_kind::{MemberKind, TyKind};
use husky_entity_route::*;
use husky_file::FilePtr;
use husky_path_utils::*;
use husky_print_utils::msg_once;
use husky_static_defn::*;
use husky_word::{dash_to_snake, CustomIdentifier, Identifier, RootIdentifier, WordPtr};
use thin_vec::{thin_vec, ThinVec};
use upcast::Upcast;

use fold::FoldableStorage;

use std::{path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntitySyntaxSalsaQueryGroup:
    husky_token::TokenQueryGroup + InternEntityRoute + ResolveStaticRootDefn
{
    fn subroute_table(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResultArc<SubrouteTable>;

    fn subentity_routes(&self, entity_route: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;
    fn subentity_kinded_routes(
        &self,
        entity_route: EntityRoutePtr,
    ) -> Arc<Vec<(EntityKind, EntityRoutePtr)>>;

    fn husky_entity_kind(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntityKind>;

    fn entity_source(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntitySource>;

    fn submodules(&self, module: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;

    fn entity_route_menu(&self) -> Arc<EntityRouteMenu>;
}

fn entity_route_menu(db: &dyn EntitySyntaxSalsaQueryGroup) -> Arc<EntityRouteMenu> {
    let std_mod = EntityRoutePtr::Root(RootIdentifier::Std);
    let std_ops_mod = db.subroute(std_mod, db.intern_word("ops").custom(), thin_vec![]);
    let std_ops_index_trai =
        db.subroute(std_ops_mod, db.intern_word("Index").custom(), thin_vec![]);
    let std_slice_mod = db.subroute(std_mod, db.intern_word("slice").custom(), thin_vec![]);
    let std_slice_cyclic_slice = db.subroute(
        std_slice_mod,
        db.intern_word("CyclicSlice").custom(),
        thin_vec![],
    );
    Arc::new(EntityRouteMenu {
        clone_trait: EntityRoutePtr::Root(RootIdentifier::CloneTrait),
        copy_trait: EntityRoutePtr::Root(RootIdentifier::CopyTrait),
        void_type: EntityRoutePtr::Root(RootIdentifier::Void),
        i32_ty: EntityRoutePtr::Root(RootIdentifier::I32),
        vec_ty: EntityRoutePtr::Root(RootIdentifier::Vec),
        std_mod,
        std_ops_mod,
        std_ops_index_trai,
        std_slice_cyclic_slice,
    })
}

fn subroute_table(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResultArc<SubrouteTable> {
    let husky_entity_kind = db.husky_entity_kind(entity_route)?;
    match db.husky_entity_kind(entity_route)? {
        EntityKind::Function { .. }
        | EntityKind::Feature
        | EntityKind::EnumVariant
        | EntityKind::Main
        | EntityKind::Member(_) => Ok(Arc::new(SubrouteTable::new(
            entity_route,
            husky_entity_kind,
        ))),
        EntityKind::Module | EntityKind::Type(_) | EntityKind::Trait => {
            Ok(Arc::new(match db.entity_source(entity_route)? {
                EntitySource::StaticModuleItem(data) => {
                    SubrouteTable::from_static(db, entity_route, husky_entity_kind, data)
                }
                EntitySource::WithinModule {
                    file,
                    token_group_index,
                } => {
                    let text = db.tokenized_text(file)?;
                    let item = text.iter_from(token_group_index).next().unwrap();
                    if let Some(children) = item.opt_children {
                        SubrouteTable::parse(db, file, entity_route, husky_entity_kind, children)
                    } else {
                        SubrouteTable::new(entity_route, husky_entity_kind)
                    }
                }
                EntitySource::Module { file: file_id } => {
                    let text = db.tokenized_text(file_id)?;
                    SubrouteTable::parse(db, file_id, entity_route, husky_entity_kind, text.iter())
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
    entity_route: EntityRoutePtr,
) -> Arc<Vec<EntityRoutePtr>> {
    Arc::new(db.subroute_table(entity_route).map_or(Vec::new(), |table| {
        table.subroute_iter(db, entity_route).collect()
    }))
}

fn subentity_kinded_routes(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<Vec<(EntityKind, EntityRoutePtr)>> {
    Arc::new(db.subroute_table(entity_route).map_or(Vec::new(), |table| {
        table
            .subentity_kinded_route_iter(db, entity_route)
            .collect()
    }))
}

fn submodules(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    module: EntityRoutePtr,
) -> Arc<Vec<EntityRoutePtr>> {
    Arc::new(
        db.subroute_table(module)
            .unwrap()
            .submodule_route_iter(db, module)
            .collect(),
    )
}

fn husky_entity_kind(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResult<EntityKind> {
    entity_kind_from_entity_route_kind(db, &entity_route.variant)
}

fn entity_kind_from_entity_route_kind(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route_variant: &EntityRouteVariant,
) -> EntitySyntaxResult<EntityKind> {
    Ok(match entity_route_variant {
        EntityRouteVariant::Root { ident } => match ident {
            RootIdentifier::Void
            | RootIdentifier::I32
            | RootIdentifier::I64
            | RootIdentifier::F32
            | RootIdentifier::F64
            | RootIdentifier::B32
            | RootIdentifier::B64
            | RootIdentifier::Bool => EntityKind::Type(TyKind::Primitive),
            RootIdentifier::Vec => EntityKind::Type(TyKind::Vec),
            RootIdentifier::Tuple => EntityKind::Type(TyKind::Tuple),
            RootIdentifier::Mor => todo!(),
            RootIdentifier::ThickFp => EntityKind::Type(TyKind::ThickFp),
            RootIdentifier::Array => todo!(),
            RootIdentifier::DatasetType => EntityKind::Type(TyKind::BoxAny),
            RootIdentifier::TraitType | RootIdentifier::TypeType | RootIdentifier::ModuleType => {
                EntityKind::Type(TyKind::HigherKind)
            }
            RootIdentifier::True | RootIdentifier::False => EntityKind::EnumVariant,
            RootIdentifier::Fn | RootIdentifier::FnMut | RootIdentifier::FnOnce => {
                EntityKind::Trait
            }
            RootIdentifier::Debug | RootIdentifier::Std | RootIdentifier::Core => {
                EntityKind::Module
            }
            RootIdentifier::Domains => EntityKind::Module,
            RootIdentifier::CloneTrait
            | RootIdentifier::CopyTrait
            | RootIdentifier::PartialEqTrait
            | RootIdentifier::EqTrait => EntityKind::Trait,
            RootIdentifier::Ref => EntityKind::Type(TyKind::Ref),
            RootIdentifier::Option => EntityKind::Type(TyKind::Option),
            RootIdentifier::VisualType => todo!(),
            RootIdentifier::RefMut => todo!(),
        },
        EntityRouteVariant::Package { .. } => EntityKind::Module,
        EntityRouteVariant::Child { parent, ident } => match parent.variant {
            EntityRouteVariant::ThisType { .. } => {
                EntityKind::Member(MemberKind::TraitAssociatedAny)
            }
            _ => db
                .subroute_table(*parent)
                .unwrap()
                .husky_entity_kind(*ident)?,
        },
        EntityRouteVariant::TargetInputValue { .. } => EntityKind::Feature,
        EntityRouteVariant::Any {
            husky_entity_kind, ..
        } => *husky_entity_kind,
        EntityRouteVariant::ThisType { .. } => EntityKind::Type(TyKind::ThisAny),
        EntityRouteVariant::TypeAsTraitMember { .. } => {
            EntityKind::Member(MemberKind::TraitAssociatedAny)
        }
        EntityRouteVariant::TargetOutputType => EntityKind::Type(TyKind::TargetOutputAny),
    })
}

fn entity_source(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResult<EntitySource> {
    if entity_route.canonicalize().kind() != CanonicalTyKind::Intrinsic {
        panic!("expect intrinsic, but get `{}`", entity_route)
    }
    match entity_route.variant {
        EntityRouteVariant::Root { ident } => Ok(EntitySource::StaticModuleItem(db
            .__root_defn_resolver()(
            ident
        ))),
        EntityRouteVariant::Package { main, .. } => Ok(EntitySource::Module { file: main }),
        EntityRouteVariant::Child { parent, ident } => {
            db.subroute_table(parent)?.entity_source(ident)
        }
        EntityRouteVariant::TargetInputValue => Ok(EntitySource::TargetInput),
        EntityRouteVariant::Any {
            ident, file, range, ..
        } => Ok(EntitySource::Any {
            route: entity_route,
            ident,
            file,
            range,
        }),
        EntityRouteVariant::ThisType { file, range } => Ok(EntitySource::ThisType { file, range }),
        EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match trai {
            EntityRoutePtr::Root(root_ident) => match root_ident {
                RootIdentifier::CloneTrait => {
                    msg_once!("ad hoc");
                    Ok(EntitySource::StaticTypeAsTraitMember)
                    // db.entity_source(EntityRoutePtr::Root(RootIdentifier::CloneTrait))
                }
                _ => todo!(),
            },
            EntityRoutePtr::Custom(_) => match trai.variant {
                EntityRouteVariant::ThisType { file, range } => {
                    let ty_source = db.entity_source(ty).unwrap();
                    match ty_source {
                        EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
                            EntityStaticDefnVariant::Ty { .. } => {
                                Ok(EntitySource::StaticTypeAsTraitMember)
                            }
                            _ => panic!(),
                        },
                        EntitySource::WithinBuiltinModule => todo!(),
                        EntitySource::WithinModule { .. } => todo!(),
                        EntitySource::Module { .. } => todo!(),
                        EntitySource::TargetInput { .. } => todo!(),
                        EntitySource::StaticTypeMember(_) => todo!(),
                        EntitySource::StaticTraitMember(_) => todo!(),
                        EntitySource::StaticTypeAsTraitMember => todo!(),
                        EntitySource::Any { .. } => todo!(),
                        EntitySource::StaticEnumVariant(_) => todo!(),
                        EntitySource::ThisType { .. } => todo!(),
                    }
                }
                _ => todo!(),
            },
        },
        EntityRouteVariant::TargetOutputType => todo!(),
    }
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
    EntitySyntaxSalsaQueryGroup + InternEntityRoute + Upcast<dyn EntitySyntaxSalsaQueryGroup>
{
    fn subroute_result(
        &self,
        parent_entity_route: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: ThinVec<SpatialArgument>,
    ) -> EntitySyntaxResult<EntityRoutePtr> {
        let parent_subscope_table = self.subroute_table(parent_entity_route)?;
        if parent_subscope_table.has_subscope(ident, &generics) {
            Ok(self.intern_entity_route(EntityRoute::subroute(
                parent_entity_route,
                ident,
                generics,
            )))
        } else {
            Err(EntitySyntaxError {
                kind: EntitySyntaxErrorKind::Query,
                dev_src: dev_src!(),
                message: format!("no such subroute"),
            })
        }
    }

    fn all_modules(&self) -> Vec<EntityRoutePtr> {
        self.all_target_entrances()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn all_source_files(&self) -> Vec<FilePtr>
    where
        Self: Sized,
    {
        self.all_target_entrances()
            .iter()
            .map(|file| self.collect_source_files(*file))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<EntityRoutePtr> {
        self.all_modules().into_iter()
    }

    fn collect_modules(&self, file: FilePtr) -> Vec<EntityRoutePtr> {
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

    fn collect_source_files(&self, target_entrance: FilePtr) -> Vec<FilePtr> {
        should!(target_entrance.ends_with("main.hsy"));
        collect_all_source_files(target_entrance.parent().unwrap().to_path_buf())
            .into_iter()
            .map(|path| self.intern_file(path))
            .collect()
    }

    fn module(&self, file: FilePtr) -> EntitySyntaxResult<EntityRoutePtr> {
        let path: PathBuf = file.to_path_buf();
        if !self.file_exists(file) {
            Err(derived_error!(format!("file doesn't exist")))?
        } else if path_has_file_name(&path, "main.hsy") {
            if let Some(pack_name) = path_parent_file_name_str(&path) {
                let snake_name = dash_to_snake(&pack_name);
                if let WordPtr::Identifier(Identifier::Custom(ident)) =
                    self.word_allocator().intern(snake_name)
                {
                    Ok(self.intern_entity_route(EntityRoute::package(file, ident)))
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
            let word = self.intern_word(path.file_stem().unwrap().to_str().unwrap());
            match word {
                WordPtr::Keyword(kw) => Err(derived_error!(format!(
                    "expect custom identifier for module name, but got keyword {} instead",
                    kw.as_str()
                ))),
                WordPtr::Opr(word_opr) => Err(derived_error!(format!(
                    "expect custom identifier for module name, but got word operator {} instead",
                    word_opr.as_str()
                ))),
                WordPtr::Identifier(ident) => match ident {
                    Identifier::Root(_) => todo!(),
                    Identifier::Custom(ident) => Ok(self.intern_entity_route(EntityRoute {
                        variant: EntityRouteVariant::Child { parent, ident },
                        temporal_arguments: thin_vec![],
                        spatial_arguments: thin_vec![],
                    })),
                    Identifier::Contextual(_) => todo!(),
                },
                WordPtr::Decorator(_) => todo!(),
                WordPtr::Pattern(_) => todo!(),
            }
        } else {
            Err(derived_error!(format!(
                "file (path: {:?}) should have extension .hsy",
                path.to_str()
            )))?
        }
    }

    fn module_file(&self, module: EntityRoutePtr) -> EntitySyntaxResult<FilePtr> {
        Ok(match self.entity_source(module)? {
            EntitySource::StaticModuleItem(_) => panic!(),
            EntitySource::WithinModule { file, .. } => file,
            EntitySource::Module { file } => file,
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::TargetInput { .. } => todo!(),
            EntitySource::StaticTypeMember(_) => todo!(),
            EntitySource::StaticTraitMember(_) => todo!(),
            EntitySource::StaticTypeAsTraitMember => todo!(),
            EntitySource::Any { .. } => todo!(),
            EntitySource::StaticEnumVariant(_) => todo!(),
            EntitySource::ThisType { .. } => todo!(),
        })
    }

    fn entity_kind_from_entity_route_variant(
        &self,
        entity_route_variant: &EntityRouteVariant,
    ) -> EntitySyntaxResult<EntityKind> {
        entity_kind_from_entity_route_kind(self.upcast(), entity_route_variant)
    }
}
