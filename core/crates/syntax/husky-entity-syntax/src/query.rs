use crate::*;
use entity_kind::{MemberKind, TyKind};
use husky_check_utils::{should, should_eq};
use husky_dev_utils::dev_src;
use husky_entity_route::*;
use husky_file::{FileError, FileErrorKind, FilePtr};
use husky_print_utils::{epin, msg_once, p};
use husky_text::TextRange;
use husky_word::{dash_to_snake, CustomIdentifier, Identifier, RootIdentifier, WordPtr};
use path_utils::*;
use static_defn::*;
use thin_vec::{thin_vec, ThinVec};
use upcast::Upcast;

use fold::FoldableStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
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

    fn entity_kind(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntityKind>;

    fn entity_source(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntitySource>;

    fn submodules(&self, module: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;
}

fn subroute_table(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResultArc<SubrouteTable> {
    let entity_kind = db.entity_kind(entity_route)?;
    match db.entity_kind(entity_route)? {
        EntityKind::Function { .. }
        | EntityKind::Feature
        | EntityKind::EnumLiteral
        | EntityKind::Main
        | EntityKind::Member(_) => Ok(Arc::new(SubrouteTable::new(entity_route, entity_kind))),
        EntityKind::Module | EntityKind::Type(_) | EntityKind::Trait => {
            Ok(Arc::new(match db.entity_source(entity_route)? {
                EntitySource::StaticModuleItem(data) => {
                    SubrouteTable::from_static(db, entity_route, entity_kind, data)
                }
                EntitySource::WithinModule {
                    file,
                    token_group_index,
                } => {
                    let text = db.tokenized_text(file)?;
                    let item = text.iter_from(token_group_index).next().unwrap();
                    if let Some(children) = item.opt_children {
                        SubrouteTable::parse(db, file, entity_route, entity_kind, children)
                    } else {
                        SubrouteTable::new(entity_route, entity_kind)
                    }
                }
                EntitySource::Module { file: file_id } => {
                    let text = db.tokenized_text(file_id)?;
                    SubrouteTable::parse(db, file_id, entity_route, entity_kind, text.iter())
                }
                EntitySource::WithinBuiltinModule => todo!(),
                EntitySource::Input { .. } => todo!(),
                EntitySource::StaticTypeMember(_) => todo!(),
                EntitySource::StaticTraitMember(_) => todo!(),
                EntitySource::StaticTypeAsTraitMember => todo!(),
                EntitySource::Generic { .. } => todo!(),
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

fn entity_kind(
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
            RootIdentifier::Fp => EntityKind::Type(TyKind::Fp),
            RootIdentifier::Array => todo!(),
            RootIdentifier::DatasetType => EntityKind::Type(TyKind::BoxAny),
            RootIdentifier::TraitType | RootIdentifier::TypeType | RootIdentifier::ModuleType => {
                EntityKind::Type(TyKind::HigherKind)
            }
            RootIdentifier::True | RootIdentifier::False => EntityKind::EnumLiteral,
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
        },
        EntityRouteVariant::Package { .. } => EntityKind::Module,
        EntityRouteVariant::Child { parent, ident } => match parent.variant {
            EntityRouteVariant::ThisType => EntityKind::Member(MemberKind::TraitAssociatedAny),
            _ => db.subroute_table(*parent).unwrap().entity_kind(*ident)?,
        },
        EntityRouteVariant::Input { .. } => EntityKind::Feature,
        EntityRouteVariant::Generic { entity_kind, .. } => *entity_kind,
        EntityRouteVariant::ThisType => EntityKind::Type(TyKind::ThisAny),
        EntityRouteVariant::TypeAsTraitMember { .. } => {
            EntityKind::Member(MemberKind::TraitAssociatedAny)
        }
    })
}

fn entity_source(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResult<EntitySource> {
    match entity_route.variant {
        EntityRouteVariant::Root { ident } => Ok(EntitySource::StaticModuleItem(db
            .__root_defn_resolver()(
            ident
        ))),
        EntityRouteVariant::Package { main, .. } => Ok(EntitySource::Module { file: main }),
        EntityRouteVariant::Child { parent, ident } => {
            db.subroute_table(parent)?.entity_source(ident)
        }
        EntityRouteVariant::Input { main } => Ok(EntitySource::Input { main_file: main }),
        EntityRouteVariant::Generic {
            ident, file, range, ..
        } => Ok(EntitySource::Generic { ident, file, range }),
        EntityRouteVariant::ThisType => panic!(),
        EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match trai {
            EntityRoutePtr::Root(root_ident) => match root_ident {
                RootIdentifier::CloneTrait => {
                    msg_once!("ad hoc");
                    Ok(EntitySource::StaticTypeAsTraitMember)
                    // db.entity_source(EntityRoutePtr::Root(RootIdentifier::CloneTrait))
                }
                _ => todo!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
            EntityRoutePtr::ThisType => {
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
                    EntitySource::Input { .. } => todo!(),
                    EntitySource::StaticTypeMember(_) => todo!(),
                    EntitySource::StaticTraitMember(_) => todo!(),
                    EntitySource::StaticTypeAsTraitMember => todo!(),
                    EntitySource::Generic { .. } => todo!(),
                }
            }
        },
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
    fn opt_package_main(&self) -> Option<FilePtr>;

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
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn all_source_files(&self) -> Vec<FilePtr>
    where
        Self: Sized,
    {
        self.all_main_files()
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

    fn collect_source_files(&self, main_file: FilePtr) -> Vec<FilePtr> {
        should!(main_file.ends_with("main.hsk"));
        collect_all_source_files(main_file.parent().unwrap().to_path_buf())
            .into_iter()
            .map(|path| self.intern_file(path))
            .collect()
    }

    fn module(&self, file: FilePtr) -> EntitySyntaxResult<EntityRoutePtr> {
        let path: PathBuf = file.to_path_buf();
        if !self.file_exists(file) {
            Err(derived_error!(format!("file doesn't exist")))?
        } else if path_has_file_name(&path, "main.hsk") {
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
        } else if path_has_extension(&path, "hsk") {
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
                    Identifier::Builtin(_) => todo!(),
                    Identifier::Custom(ident) => Ok(self.intern_entity_route(EntityRoute {
                        variant: EntityRouteVariant::Child { parent, ident },
                        temporal_arguments: thin_vec![],
                        spatial_arguments: thin_vec![],
                    })),
                    Identifier::Contextual(_) => todo!(),
                },
                WordPtr::Decorator(_) => todo!(),
            }
        } else {
            Err(derived_error!(format!(
                "file (path: {:?}) should have extension .hsk",
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
            EntitySource::Input { .. } => todo!(),
            EntitySource::StaticTypeMember(_) => todo!(),
            EntitySource::StaticTraitMember(_) => todo!(),
            EntitySource::StaticTypeAsTraitMember => todo!(),
            EntitySource::Generic { .. } => todo!(),
        })
    }

    fn entity_kind_from_entity_route_variant(
        &self,
        entity_route_variant: &EntityRouteVariant,
    ) -> EntitySyntaxResult<EntityKind> {
        entity_kind_from_entity_route_kind(self.upcast(), entity_route_variant)
    }
}
