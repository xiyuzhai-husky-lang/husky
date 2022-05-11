use crate::{error::err, *};
use check_utils::{should, should_eq};
use dev_utils::dev_src;
use entity_kind::{MemberKind, TyKind};
use entity_route::*;
use file::{FileError, FileErrorKind, FilePtr};
use path_utils::*;
use print_utils::p;
use static_defn::*;
use upcast::Upcast;
use word::{dash_to_snake, CustomIdentifier, Identifier, RootIdentifier, WordPtr};

use fold::FoldStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntityRouteSalsaQueryGroup: token::TokenQueryGroup + AllocateUniqueScope {
    fn subroute_table(&self, scope_id: EntityRoutePtr) -> EntityRouteResultArc<ChildRouteTable>;

    fn subscopes(&self, scope: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;

    fn raw_entity_kind(&self, scope_id: EntityRoutePtr) -> EntityKind;

    fn entity_source(&self, scope_id: EntityRoutePtr) -> EntityRouteResult<EntitySource>;

    fn entity_route_menu(&self) -> Arc<EntityRouteMenu>;
}

fn subroute_table(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope_id: EntityRoutePtr,
) -> EntityRouteResultArc<ChildRouteTable> {
    Ok(Arc::new(match db.entity_source(scope_id)? {
        EntitySource::StaticModuleItem(data) => ChildRouteTable::from_static(db, data),
        EntitySource::WithinModule {
            file: file_id,
            token_group_index,
        } => {
            let text = db.tokenized_text(file_id)?;
            let item = text.iter_from(token_group_index).next().unwrap();
            if let Some(children) = item.children {
                ChildRouteTable::parse(file_id, children)
            } else {
                ChildRouteTable::empty()
            }
        }
        EntitySource::Module { file: file_id } => {
            let text = db.tokenized_text(file_id)?;
            ChildRouteTable::parse(file_id, text.iter())
        }
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
        EntitySource::StaticTypeAsTraitMember => todo!(),
    }))
}

fn subscopes(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope: EntityRoutePtr,
) -> Arc<Vec<EntityRoutePtr>> {
    Arc::new(db.subroute_table(scope).map_or(Vec::new(), |table| {
        table
            .child_routes(scope)
            .into_iter()
            .map(|scope| db.intern_entity_route(scope))
            .collect()
    }))
}

fn raw_entity_kind(db: &dyn EntityRouteSalsaQueryGroup, scope: EntityRoutePtr) -> EntityKind {
    entity_kind_from_entity_route_kind(db, scope.kind)
}

fn entity_kind_from_entity_route_kind(
    db: &dyn EntityRouteSalsaQueryGroup,
    entity_route_kind: EntityRouteKind,
) -> EntityKind {
    match entity_route_kind {
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
            | RootIdentifier::DatasetType => EntityKind::Type(TyKind::Other),
            RootIdentifier::True | RootIdentifier::False => EntityKind::Literal,
            RootIdentifier::Fn | RootIdentifier::FnMut | RootIdentifier::FnOnce => {
                EntityKind::Trait
            }
            RootIdentifier::Debug | RootIdentifier::Std | RootIdentifier::Core => {
                EntityKind::Module
            }
            RootIdentifier::Type => todo!(),
            RootIdentifier::Datasets => EntityKind::Module,
            RootIdentifier::CloneTrait
            | RootIdentifier::CopyTrait
            | RootIdentifier::PartialEqTrait
            | RootIdentifier::EqTrait => EntityKind::Trait,
        },
        EntityRouteKind::Package { .. } => EntityKind::Module,
        EntityRouteKind::Child { parent, ident } => match parent.kind {
            EntityRouteKind::ThisType => EntityKind::Member(MemberKind::TraitAssociatedAny),
            _ => db
                .subroute_table(parent)
                .unwrap()
                .raw_entity_kind(ident)
                .unwrap(),
        },
        EntityRouteKind::Input { .. } => EntityKind::Feature,
        EntityRouteKind::Generic { entity_kind, .. } => entity_kind,
        EntityRouteKind::ThisType => EntityKind::Type(TyKind::Other),
        EntityRouteKind::TypeAsTraitMember {
            ty: parent,
            trai,
            ident,
        } => todo!(),
    }
}

fn entity_source(
    this: &dyn EntityRouteSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> EntityRouteResult<EntitySource> {
    match entity_route.kind {
        EntityRouteKind::Root { ident } => {
            Ok(EntitySource::StaticModuleItem(static_root_defn(ident)))
        }
        EntityRouteKind::Package { main, .. } => Ok(EntitySource::Module { file: main }),
        EntityRouteKind::Child { parent, ident } => {
            this.subroute_table(parent)?.entity_source(ident)
        }
        EntityRouteKind::Input { main } => Ok(EntitySource::Input { main }),
        EntityRouteKind::Generic { .. } => todo!(),
        EntityRouteKind::ThisType => panic!(),
        EntityRouteKind::TypeAsTraitMember { ty, .. } => {
            let ty_source = this.entity_source(ty).unwrap();
            match ty_source {
                EntitySource::StaticModuleItem(static_defn) => match static_defn.variant {
                    EntityStaticDefnVariant::Type { .. } => {
                        Ok(EntitySource::StaticTypeAsTraitMember)
                    }
                    _ => panic!(),
                },
                EntitySource::WithinBuiltinModule => todo!(),
                EntitySource::WithinModule { .. } => todo!(),
                EntitySource::Module { .. } => todo!(),
                EntitySource::Input { .. } => todo!(),
                EntitySource::StaticTypeMember => todo!(),
                EntitySource::StaticTypeAsTraitMember => todo!(),
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
        RootIdentifier::Type => todo!(),
        RootIdentifier::CloneTrait => &CLONE_TRAIT_DEFN,
        RootIdentifier::CopyTrait => todo!(),
        RootIdentifier::PartialEqTrait => todo!(),
        RootIdentifier::EqTrait => todo!(),
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
    fn child_route(
        &self,
        parent_scope: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> Option<EntityRoutePtr> {
        let parent_subscope_table = self.subroute_table(parent_scope);
        if parent_subscope_table.map_or(false, |table| table.has_subscope(ident, &generics)) {
            Some(self.intern_entity_route(EntityRoute::child_route(parent_scope, ident, generics)))
        } else {
            None
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

    fn module(&self, file: FilePtr) -> EntityRouteResult<EntityRoutePtr> {
        let path: PathBuf = file.to_path_buf();
        if !self.file_exists(file) {
            err!(format!("file doesn't exist"))?
        } else if path_has_file_name(&path, "main.hsk") {
            if let Some(pack_name) = path_parent_file_name_str(&path) {
                let snake_name = dash_to_snake(&pack_name);
                if let WordPtr::Identifier(Identifier::Custom(ident)) =
                    self.word_allocator().alloc(snake_name)
                {
                    Ok(self.intern_entity_route(EntityRoute::pack(file, ident)))
                } else {
                    err!(format!("pack name should be identifier"))?
                }
            } else {
                err!(format!("pack root should have filename"))?
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
                WordPtr::Keyword(kw) => {
                    err!(format!(
                        "expect custom identifier for module name, but got keyword {} instead",
                        kw.as_str()
                    ))
                }
                WordPtr::Identifier(ident) => match ident {
                    Identifier::Builtin(_) => todo!(),
                    Identifier::Custom(ident) => Ok(self.intern_entity_route(EntityRoute {
                        kind: EntityRouteKind::Child { parent, ident },
                        generic_arguments: vec![],
                    })),
                    Identifier::Contextual(_) => todo!(),
                },
            }
        } else {
            err!(format!(
                "file (path: {:?}) should have extension .hsk",
                path.to_str()
            ))?
        }
    }

    fn module_file(&self, module: EntityRoutePtr) -> EntityRouteResult<FilePtr> {
        Ok(match self.entity_source(module)? {
            EntitySource::StaticModuleItem(_) => panic!(),
            EntitySource::WithinModule { file: file_id, .. } => file_id,
            EntitySource::Module { file: file_id } => file_id,
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::Input { .. } => todo!(),
            EntitySource::StaticTypeMember => todo!(),
            EntitySource::StaticTypeAsTraitMember => todo!(),
        })
    }

    fn submodule_file_id(
        &self,
        parent_id: FilePtr,
        ident: CustomIdentifier,
    ) -> EntityRouteResult<FilePtr> {
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

    fn raw_entity_kind_from_scope_kind(&self, scope_kind: EntityRouteKind) -> EntityKind {
        entity_kind_from_entity_route_kind(self.upcast(), scope_kind)
    }
}
