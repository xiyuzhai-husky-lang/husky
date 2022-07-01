use crate::*;
use check_utils::{should, should_eq};
use dev_utils::dev_src;
use entity_kind::{MemberKind, TyKind};
use husky_entity_route_syntax::*;
use husky_file::{FileError, FileErrorKind, FilePtr};
use husky_text::TextRange;
use path_utils::*;
use print_utils::{epin, msg_once, p};
use static_defn::*;
use thin_vec::{thin_vec, ThinVec};
use upcast::Upcast;
use word::{dash_to_snake, CustomIdentifier, Identifier, RootIdentifier, WordPtr};

use fold::FoldableStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntitySyntaxSalsaQueryGroup:
    husky_token::TokenQueryGroup + AllocateUniqueScope + ResolveStaticRootDefn
{
    fn subroute_table(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResultArc<SubrouteTable>;

    fn subscopes(&self, entity_route: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;

    fn entity_kind(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntityKind>;

    fn entity_locus(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntityLocus>;

    fn entity_route_menu(&self) -> Arc<EntityRouteMenu>;
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
            Ok(Arc::new(match db.entity_locus(entity_route)? {
                EntityLocus::StaticModuleItem(data) => {
                    SubrouteTable::from_static(db, entity_route, entity_kind, data)
                }
                EntityLocus::WithinModule {
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
                EntityLocus::Module { file: file_id } => {
                    let text = db.tokenized_text(file_id)?;
                    SubrouteTable::parse(db, file_id, entity_route, entity_kind, text.iter())
                }
                EntityLocus::WithinBuiltinModule => todo!(),
                EntityLocus::Input { .. } => todo!(),
                EntityLocus::StaticTypeMember => todo!(),
                EntityLocus::StaticTypeAsTraitMember => todo!(),
            }))
        }
    }
}

fn subscopes(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<Vec<EntityRoutePtr>> {
    Arc::new(db.subroute_table(entity_route).map_or(Vec::new(), |table| {
        table.subroute_iter(db, entity_route).collect()
    }))
}

fn entity_kind(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResult<EntityKind> {
    entity_kind_from_entity_route_kind(db, entity_route.kind)
}

fn entity_kind_from_entity_route_kind(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route_kind: EntityRouteKind,
) -> EntitySyntaxResult<EntityKind> {
    Ok(match entity_route_kind {
        EntityRouteKind::Root { ident } => match ident {
            RootIdentifier::Void
            | RootIdentifier::I32
            | RootIdentifier::F32
            | RootIdentifier::B32
            | RootIdentifier::B64
            | RootIdentifier::Bool => EntityKind::Type(TyKind::Primitive),
            RootIdentifier::Vec => EntityKind::Type(TyKind::Vec),
            RootIdentifier::Tuple
            | RootIdentifier::Mor
            | RootIdentifier::Fp
            | RootIdentifier::Array
            | RootIdentifier::DatasetType
            | RootIdentifier::TraitType
            | RootIdentifier::TypeType
            | RootIdentifier::ModuleType => EntityKind::Type(TyKind::Other),
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
            RootIdentifier::Ref => todo!(),
            RootIdentifier::VisualType => todo!(),
        },
        EntityRouteKind::Package { .. } => EntityKind::Module,
        EntityRouteKind::Child { parent, ident } => match parent.kind {
            EntityRouteKind::ThisType => EntityKind::Member(MemberKind::TraitAssociatedAny),
            _ => db.subroute_table(parent).unwrap().entity_kind(ident)?,
        },
        EntityRouteKind::Input { .. } => EntityKind::Feature,
        EntityRouteKind::Generic { entity_kind, .. } => entity_kind,
        EntityRouteKind::ThisType => EntityKind::Type(TyKind::Other),
        EntityRouteKind::TypeAsTraitMember {
            ty: parent,
            trai,
            ident,
        } => todo!(),
    })
}

fn entity_locus(
    db: &dyn EntitySyntaxSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResult<EntityLocus> {
    match entity_route.kind {
        EntityRouteKind::Root { ident } => Ok(EntityLocus::StaticModuleItem(db
            .__root_defn_resolver()(
            ident
        ))),
        EntityRouteKind::Package { main, .. } => Ok(EntityLocus::Module { file: main }),
        EntityRouteKind::Child { parent, ident } => db.subroute_table(parent)?.entity_locus(ident),
        EntityRouteKind::Input { main } => Ok(EntityLocus::Input { main }),
        EntityRouteKind::Generic { .. } => {
            p!(entity_route);
            todo!()
        }
        EntityRouteKind::ThisType => panic!(),
        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => match trai {
            EntityRoutePtr::Root(root_ident) => match root_ident {
                RootIdentifier::CloneTrait => {
                    msg_once!("ad hoc");
                    Ok(EntityLocus::StaticTypeAsTraitMember)
                    // db.entity_locus(EntityRoutePtr::Root(RootIdentifier::CloneTrait))
                }
                _ => todo!(),
            },
            EntityRoutePtr::Custom(_) => todo!(),
            EntityRoutePtr::ThisType => {
                let ty_source = db.entity_locus(ty).unwrap();
                match ty_source {
                    EntityLocus::StaticModuleItem(static_defn) => match static_defn.variant {
                        EntityStaticDefnVariant::Ty { .. } => {
                            Ok(EntityLocus::StaticTypeAsTraitMember)
                        }
                        _ => panic!(),
                    },
                    EntityLocus::WithinBuiltinModule => todo!(),
                    EntityLocus::WithinModule { .. } => todo!(),
                    EntityLocus::Module { .. } => todo!(),
                    EntityLocus::Input { .. } => todo!(),
                    EntityLocus::StaticTypeMember => todo!(),
                    EntityLocus::StaticTypeAsTraitMember => todo!(),
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
    EntitySyntaxSalsaQueryGroup + AllocateUniqueScope + Upcast<dyn EntitySyntaxSalsaQueryGroup>
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
                WordPtr::RawOpnVariant(word_opr) => Err(derived_error!(format!(
                    "expect custom identifier for module name, but got word operator {} instead",
                    word_opr.as_str()
                ))),
                WordPtr::Identifier(ident) => match ident {
                    Identifier::Builtin(_) => todo!(),
                    Identifier::Custom(ident) => Ok(self.intern_entity_route(EntityRoute {
                        kind: EntityRouteKind::Child { parent, ident },
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
        Ok(match self.entity_locus(module)? {
            EntityLocus::StaticModuleItem(_) => panic!(),
            EntityLocus::WithinModule { file, .. } => file,
            EntityLocus::Module { file } => file,
            EntityLocus::WithinBuiltinModule => todo!(),
            EntityLocus::Input { .. } => todo!(),
            EntityLocus::StaticTypeMember => todo!(),
            EntityLocus::StaticTypeAsTraitMember => todo!(),
        })
    }

    fn entity_kind_from_scope_kind(
        &self,
        scope_kind: EntityRouteKind,
    ) -> EntitySyntaxResult<EntityKind> {
        entity_kind_from_entity_route_kind(self.upcast(), scope_kind)
    }
}
