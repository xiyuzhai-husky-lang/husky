use crate::*;
use check_utils::{should, should_eq};
use dev_utils::dev_src;
use entity_kind::{MemberKind, TyKind};
use entity_route::*;
use file::{FileError, FileErrorKind, FilePtr};
use path_utils::*;
use print_utils::p;
use static_defn::*;
use text::TextRange;
use upcast::Upcast;
use word::{dash_to_snake, CustomIdentifier, Identifier, RootIdentifier, WordPtr};

use fold::FoldStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntityRouteSalsaQueryGroup: token::TokenQueryGroup + AllocateUniqueScope {
    fn subroute_table(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResultArc<SubrouteTable>;

    fn subscopes(&self, entity_route: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;

    fn entity_kind(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntityKind>;

    fn entity_locus(&self, entity_route: EntityRoutePtr) -> EntitySyntaxResult<EntityLocus>;

    fn entity_route_menu(&self) -> Arc<EntityRouteMenu>;
}

fn subroute_table(
    db: &dyn EntityRouteSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResultArc<SubrouteTable> {
    let entity_kind = db.entity_kind(entity_route)?;
    match db.entity_kind(entity_route)? {
        EntityKind::Routine
        | EntityKind::Feature
        | EntityKind::Pattern
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
    db: &dyn EntityRouteSalsaQueryGroup,
    scope: EntityRoutePtr,
) -> Arc<Vec<EntityRoutePtr>> {
    Arc::new(
        db.subroute_table(scope)
            .map_or(Vec::new(), |table| table.subroute_iter(db, scope).collect()),
    )
}

fn entity_kind(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope: EntityRoutePtr,
) -> EntitySyntaxResult<EntityKind> {
    entity_kind_from_entity_route_kind(db, scope.kind)
}

fn entity_kind_from_entity_route_kind(
    db: &dyn EntityRouteSalsaQueryGroup,
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
            | RootIdentifier::Fp
            | RootIdentifier::Array
            | RootIdentifier::DatasetType
            | RootIdentifier::TypeType
            | RootIdentifier::ModuleType => EntityKind::Type(TyKind::Other),
            RootIdentifier::True | RootIdentifier::False => EntityKind::EnumLiteral,
            RootIdentifier::Fn | RootIdentifier::FnMut | RootIdentifier::FnOnce => {
                EntityKind::Trait
            }
            RootIdentifier::Debug | RootIdentifier::Std | RootIdentifier::Core => {
                EntityKind::Module
            }
            RootIdentifier::Datasets => EntityKind::Module,
            RootIdentifier::CloneTrait
            | RootIdentifier::CopyTrait
            | RootIdentifier::PartialEqTrait
            | RootIdentifier::EqTrait => EntityKind::Trait,
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
    this: &dyn EntityRouteSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntitySyntaxResult<EntityLocus> {
    match entity_route.kind {
        EntityRouteKind::Root { ident } => {
            Ok(EntityLocus::StaticModuleItem(static_root_defn(ident)))
        }
        EntityRouteKind::Package { main, .. } => Ok(EntityLocus::Module { file: main }),
        EntityRouteKind::Child { parent, ident } => {
            this.subroute_table(parent)?.entity_locus(ident)
        }
        EntityRouteKind::Input { main } => Ok(EntityLocus::Input { main }),
        EntityRouteKind::Generic { .. } => todo!(),
        EntityRouteKind::ThisType => panic!(),
        EntityRouteKind::TypeAsTraitMember { ty, .. } => {
            let ty_source = this.entity_locus(ty).unwrap();
            match ty_source {
                EntityLocus::StaticModuleItem(static_defn) => match static_defn.variant {
                    EntityStaticDefnVariant::Type { .. } => {
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
    }
}

pub fn static_root_defn(ident: RootIdentifier) -> &'static EntityStaticDefn {
    match ident {
        RootIdentifier::Void => &VOID_TYPE_DEFN,
        RootIdentifier::I32 => &I32_TYPE_DEFN,
        RootIdentifier::F32 => &F32_TYPE_DEFN,
        RootIdentifier::B32 => &B32_TYPE_DEFN,
        RootIdentifier::B64 => &B64_TYPE_DEFN,
        RootIdentifier::Bool => &BOOL_TYPE_DEFN,
        RootIdentifier::True => todo!(),
        RootIdentifier::False => todo!(),
        RootIdentifier::Vec => &VEC_TYPE_DEFN,
        RootIdentifier::Tuple => todo!(),
        RootIdentifier::Debug => todo!(),
        RootIdentifier::Std => &STD_MODULE_DEFN,
        RootIdentifier::Core => todo!(),
        RootIdentifier::Fp => todo!(),
        RootIdentifier::Fn => todo!(),
        RootIdentifier::FnMut => todo!(),
        RootIdentifier::FnOnce => todo!(),
        RootIdentifier::Array => todo!(),
        RootIdentifier::Datasets => datasets::DATASETS_MODULE_DEFN,
        RootIdentifier::DatasetType => &datasets::DATASET_TYPE_DEFN,
        RootIdentifier::TypeType => todo!(),
        RootIdentifier::CloneTrait => &CLONE_TRAIT_DEFN,
        RootIdentifier::CopyTrait => todo!(),
        RootIdentifier::PartialEqTrait => todo!(),
        RootIdentifier::EqTrait => todo!(),
        RootIdentifier::ModuleType => todo!(),
    }
    .into()
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

pub trait EntityRouteQueryGroup:
    EntityRouteSalsaQueryGroup + AllocateUniqueScope + Upcast<dyn EntityRouteSalsaQueryGroup>
{
    fn subroute_result(
        &self,
        parent_scope: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> EntitySyntaxResult<EntityRoutePtr> {
        let parent_subscope_table = self.subroute_table(parent_scope)?;
        if parent_subscope_table.has_subscope(ident, &generics) {
            Ok(self.intern_entity_route(EntityRoute::subroute(parent_scope, ident, generics)))
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
                            self.submodule_file_id(file, ident)
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
                    self.word_allocator().alloc(snake_name)
                {
                    Ok(self.intern_entity_route(EntityRoute::package(file, ident)))
                } else {
                    Err(derived_error!(format!("pack name should be identifier")))?
                }
            } else {
                Err(derived_error!(format!("pack root should have filename")))?
            }
        } else if path_has_file_name(&path, "mod.hsk") {
            todo!()
        } else if path_has_extension(&path, "hsk") {
            let parent = {
                let maybe_main_path = path.with_file_name("main.hsk");
                if maybe_main_path.exists() {
                    self.module(self.intern_file(maybe_main_path))?
                } else {
                    todo!()
                }
            };
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
                        kind: EntityRouteKind::Child { parent, ident },
                        generic_arguments: vec![],
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

    fn submodule_file_id(
        &self,
        parent_id: FilePtr,
        ident: CustomIdentifier,
    ) -> EntitySyntaxResult<FilePtr> {
        let path = &*parent_id;

        should!(path_has_file_name(&path, "mod.hsk") || path_has_file_name(path, "main.hsk"));

        let module_path1 = path.with_file_name(format!("{}.hsk", ident.deref()));
        let module_path2 = path.with_file_name(format!("{}/mod.hsk", ident.deref()));

        let module_path = if module_path1.is_file() && !module_path2.is_file() {
            Ok(module_path1)
        } else if module_path2.is_file() && !module_path1.is_file() {
            Ok(module_path1)
        } else {
            Err(FileError {
                kind: FileErrorKind::FileNotFound,
                dev_src: dev_src!(),
            }
            .into())
        };

        module_path.map(|pth| self.intern_file(pth))
    }

    fn entity_kind_from_scope_kind(
        &self,
        scope_kind: EntityRouteKind,
    ) -> EntitySyntaxResult<EntityKind> {
        entity_kind_from_entity_route_kind(self.upcast(), scope_kind)
    }
}
