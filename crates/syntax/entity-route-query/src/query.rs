use crate::{error::scope_err, *};
use check_utils::should;
use entity_route::*;
use file::FilePtr;
use path_utils::*;

use entity_kind::TyKind;
use static_defn::*;
use upcast::Upcast;
use word::{dash_to_snake, CustomIdentifier, Identifier, RootIdentifier, WordPtr};

use fold::FoldStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntityRouteSalsaQueryGroup: token::TokenQueryGroup + AllocateUniqueScope {
    fn subscope_table(&self, scope_id: EntityRoutePtr) -> ScopeResultArc<SubscopeTable>;

    fn subscopes(&self, scope: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;

    fn raw_entity_kind(&self, scope_id: EntityRoutePtr) -> EntityKind;

    fn entity_source(&self, scope_id: EntityRoutePtr) -> ScopeResult<EntitySource>;

    fn entity_route_menu(&self) -> Arc<EntityRouteMenu>;
}

fn subscope_table(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope_id: EntityRoutePtr,
) -> ScopeResultArc<SubscopeTable> {
    Ok(Arc::new(match db.entity_source(scope_id)? {
        EntitySource::StaticModuleItem(data) => SubscopeTable::from_static(db, data),
        EntitySource::WithinModule {
            file: file_id,
            token_group_index,
        } => {
            let text = db.tokenized_text(file_id)?;
            let item = text.fold_iter(token_group_index).next().unwrap();
            if let Some(children) = item.children {
                SubscopeTable::parse(file_id, children)
            } else {
                SubscopeTable::empty()
            }
        }
        EntitySource::Module { file: file_id } => {
            let text = db.tokenized_text(file_id)?;
            SubscopeTable::parse(file_id, text.fold_iter(0))
        }
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
    }))
}

fn subscopes(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope: EntityRoutePtr,
) -> Arc<Vec<EntityRoutePtr>> {
    Arc::new(db.subscope_table(scope).map_or(Vec::new(), |table| {
        table
            .subscopes(scope)
            .into_iter()
            .map(|scope| db.intern_entity_route(scope))
            .collect()
    }))
}

fn raw_entity_kind(db: &dyn EntityRouteSalsaQueryGroup, scope: EntityRoutePtr) -> EntityKind {
    entity_kind_from_scope_kind(db, scope.kind)
}

fn entity_kind_from_scope_kind(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope_kind: EntityRouteKind,
) -> EntityKind {
    match scope_kind {
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
            EntityRouteKind::ThisType => EntityKind::Member,
            _ => db
                .subscope_table(parent)
                .unwrap()
                .raw_entity_kind(ident)
                .unwrap(),
        },
        EntityRouteKind::Input { .. } => EntityKind::Feature,
        EntityRouteKind::Generic { entity_kind, .. } => entity_kind,
        EntityRouteKind::ThisType => EntityKind::Type(TyKind::Other),
        EntityRouteKind::TraitMember {
            ty: parent,
            trai,
            ident,
        } => todo!(),
    }
}

fn entity_source(
    this: &dyn EntityRouteSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> ScopeResult<EntitySource> {
    match entity_route.kind {
        EntityRouteKind::Root { ident } => {
            Ok(EntitySource::StaticModuleItem(static_root_defn(ident)))
        }
        EntityRouteKind::Package { main, .. } => Ok(EntitySource::Module { file: main }),
        EntityRouteKind::Child { parent, ident } => {
            this.subscope_table(parent)?.scope_source(ident)
        }
        EntityRouteKind::Input { main } => Ok(EntitySource::Input { main }),
        EntityRouteKind::Generic { .. } => todo!(),
        EntityRouteKind::ThisType => panic!(),
        EntityRouteKind::TraitMember { ty, .. } => {
            let ty_source = this.entity_source(ty).unwrap();
            match ty_source {
                EntitySource::StaticModuleItem(_) => Ok(ty_source),
                EntitySource::WithinBuiltinModule => todo!(),
                EntitySource::WithinModule { .. } => todo!(),
                EntitySource::Module { .. } => todo!(),
                EntitySource::Input { .. } => todo!(),
                EntitySource::StaticTypeMember => todo!(),
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
    fn subscope(
        &self,
        parent_scope: EntityRoutePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> Option<EntityRoutePtr> {
        let parent_subscope_table = self.subscope_table(parent_scope);
        if parent_subscope_table.map_or(false, |table| table.has_subscope(ident, &generics)) {
            Some(self.intern_entity_route(EntityRoute::child_route(parent_scope, ident, generics)))
        } else {
            None
        }
    }

    fn all_modules(&self) -> Vec<EntityRoutePtr>
    where
        Self: Sized,
    {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<EntityRoutePtr>
    where
        Self: Sized,
    {
        self.all_modules().into_iter()
    }

    fn collect_modules(&self, id: FilePtr) -> Vec<EntityRoutePtr>
    where
        Self: Sized,
    {
        if let Ok(module) = self.module(id) {
            let mut modules = vec![module];
            self.subscope_table(module).ok().map(|table| {
                modules.extend(
                    table
                        .submodule_idents()
                        .into_iter()
                        .filter_map(|ident| {
                            self.submodule_file_id(id, ident)
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

    fn module(&self, id: FilePtr) -> ScopeResult<EntityRoutePtr> {
        let path: PathBuf = (*id).into();
        if !self.file_exists(id) {
            scope_err!(format!("file didn't exist"))?
        } else if path_has_file_name(&path, "main.hsk") {
            if let Some(pack_name) = path_parent_file_name_str(&path) {
                let snake_name = dash_to_snake(&pack_name);
                if let WordPtr::Identifier(Identifier::Custom(ident)) =
                    self.word_allocator().alloc(snake_name)
                {
                    Ok(self.intern_entity_route(EntityRoute::pack(id, ident)))
                } else {
                    scope_err!(format!("pack name should be identifier"))?
                }
            } else {
                scope_err!(format!("pack root should have filename"))?
            }
        } else if path_has_file_name(&path, "mod.hsk") {
            todo!()
        } else if path_has_extension(&path, "hsk") {
            let maybe_main_path = path.with_file_name("main.hsk");
            if maybe_main_path.exists() {
                let _parent = self.module(self.intern_file(path.with_file_name("mod.hsk")));
                todo!()
            } else {
                todo!()
            }
        } else {
            scope_err!(format!(
                "file (path: {:?}) should have extension .hsk",
                path.to_str()
            ))?
        }
    }

    fn module_file(&self, module: EntityRoutePtr) -> ScopeResult<FilePtr> {
        Ok(match self.entity_source(module)? {
            EntitySource::StaticModuleItem(_) => panic!(),
            EntitySource::WithinModule { file: file_id, .. } => file_id,
            EntitySource::Module { file: file_id } => file_id,
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::Input { .. } => todo!(),
            EntitySource::StaticTypeMember => todo!(),
        })
    }

    fn submodule_file_id(&self, parent_id: FilePtr, ident: CustomIdentifier) -> ScopeResult<FilePtr>
    where
        Self: Sized,
    {
        let path = &*parent_id;

        should!(path_has_file_name(&path, "mod.hsk") || path_has_file_name(path, "main.hsk"));

        let module_path1 = path.with_file_name(format!("{}.hsk", ident.deref()));
        let module_path2 = path.with_file_name(format!("{}/mod.hsk", ident.deref()));

        let module_path = if module_path1.is_file() && !module_path2.is_file() {
            Ok(module_path1)
        } else if module_path2.is_file() && !module_path1.is_file() {
            Ok(module_path1)
        } else {
            Err(file::FileError::FileNotFound.into())
        };

        module_path.map(|pth| self.intern_file(pth))
    }

    fn raw_entity_kind_from_scope_kind(&self, scope_kind: EntityRouteKind) -> EntityKind {
        entity_kind_from_scope_kind(self.upcast(), scope_kind)
    }
}
