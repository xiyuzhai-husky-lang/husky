use crate::{error::scope_err, *};
use check_utils::should;
use entity_route::*;
use file::FilePtr;
use path_utils::*;

use entity_syntax::RawTyKind;
use upcast::Upcast;
use visual_syntax::{BuiltinVisualizer, TRIVIAL_VISUALIZER};
use word::{
    dash_to_snake, BuiltinIdentifier, ContextualIdentifier, CustomIdentifier, Identifier, WordPtr,
};

use fold::FoldStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait EntityRouteSalsaQueryGroup: token::TokenQueryGroup + AllocateUniqueScope {
    fn subscope_table(&self, scope_id: EntityRoutePtr) -> ScopeResultArc<SubscopeTable>;

    fn subscopes(&self, scope: EntityRoutePtr) -> Arc<Vec<EntityRoutePtr>>;

    fn raw_entity_kind(&self, scope_id: EntityRoutePtr) -> RawEntityKind;

    fn entity_source(&self, scope_id: EntityRoutePtr) -> ScopeResult<EntitySource>;

    fn entity_route_menu(&self) -> Arc<EntityRouteMenu>;
}

fn subscope_table(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope_id: EntityRoutePtr,
) -> ScopeResultArc<SubscopeTable> {
    Ok(Arc::new(match db.entity_source(scope_id)? {
        EntitySource::Builtin(data) => SubscopeTable::builtin(db, data),
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
        EntitySource::Contextual { .. } => todo!(),
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
            .map(|scope| db.intern_scope(scope))
            .collect()
    }))
}

fn raw_entity_kind(db: &dyn EntityRouteSalsaQueryGroup, scope: EntityRoutePtr) -> RawEntityKind {
    raw_entity_kind_from_scope_kind(db, &scope.kind)
}

fn raw_entity_kind_from_scope_kind(
    db: &dyn EntityRouteSalsaQueryGroup,
    scope_kind: &EntityRouteKind,
) -> RawEntityKind {
    match scope_kind {
        EntityRouteKind::Builtin { ident } => match ident {
            BuiltinIdentifier::Void
            | BuiltinIdentifier::I32
            | BuiltinIdentifier::F32
            | BuiltinIdentifier::B32
            | BuiltinIdentifier::B64
            | BuiltinIdentifier::Bool => RawEntityKind::Type(RawTyKind::Primitive),
            BuiltinIdentifier::Vec
            | BuiltinIdentifier::Tuple
            | BuiltinIdentifier::Fp
            | BuiltinIdentifier::Array
            | BuiltinIdentifier::DatasetType => RawEntityKind::Type(RawTyKind::Other),
            BuiltinIdentifier::True | BuiltinIdentifier::False => RawEntityKind::Literal,
            BuiltinIdentifier::Fn | BuiltinIdentifier::FnMut | BuiltinIdentifier::FnOnce => {
                RawEntityKind::Trait
            }
            BuiltinIdentifier::Debug | BuiltinIdentifier::Std | BuiltinIdentifier::Core => {
                RawEntityKind::Module
            }
            BuiltinIdentifier::Type => todo!(),
            BuiltinIdentifier::Datasets => RawEntityKind::Module,
            BuiltinIdentifier::CloneTrait
            | BuiltinIdentifier::CopyTrait
            | BuiltinIdentifier::PartialEqTrait
            | BuiltinIdentifier::EqTrait => RawEntityKind::Trait,
        },
        EntityRouteKind::pack { .. } => RawEntityKind::Module,
        EntityRouteKind::ChildScope { parent, ident } => db
            .subscope_table(*parent)
            .unwrap()
            .raw_entity_kind(*ident)
            .unwrap(),
        EntityRouteKind::Contextual { ident, .. } => match ident {
            ContextualIdentifier::Input => RawEntityKind::Feature,
            ContextualIdentifier::ThisData => todo!(),
            ContextualIdentifier::ThisType => todo!(),
        },
        EntityRouteKind::Generic {
            ident,
            raw_entity_kind,
        } => *raw_entity_kind,
    }
}

fn entity_source(
    this: &dyn EntityRouteSalsaQueryGroup,
    entity_route: EntityRoutePtr,
) -> ScopeResult<EntitySource> {
    Ok(match entity_route.kind {
        EntityRouteKind::Builtin { ident } => match ident {
            BuiltinIdentifier::Void => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Ty {
                    raw_ty_kind: RawTyKind::Primitive,
                    visualizer: TRIVIAL_VISUALIZER,
                },
            },
            BuiltinIdentifier::I32 => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Ty {
                    raw_ty_kind: RawTyKind::Primitive,
                    visualizer: TRIVIAL_VISUALIZER,
                },
            },
            BuiltinIdentifier::F32 => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Ty {
                    raw_ty_kind: RawTyKind::Primitive,
                    visualizer: TRIVIAL_VISUALIZER,
                },
            },
            BuiltinIdentifier::B32 => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Ty {
                    raw_ty_kind: RawTyKind::Primitive,
                    visualizer: TRIVIAL_VISUALIZER,
                },
            },
            BuiltinIdentifier::B64 => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Ty {
                    raw_ty_kind: RawTyKind::Primitive,
                    visualizer: TRIVIAL_VISUALIZER,
                },
            },
            BuiltinIdentifier::Bool => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Ty {
                    raw_ty_kind: RawTyKind::Primitive,
                    visualizer: TRIVIAL_VISUALIZER,
                },
            },
            BuiltinIdentifier::True => todo!(),
            BuiltinIdentifier::False => todo!(),
            BuiltinIdentifier::Vec => &BuiltinEntityData {
                subscopes: &[],
                decl: BuiltinEntityDecl::Vec,
            },
            BuiltinIdentifier::Tuple => todo!(),
            BuiltinIdentifier::Debug => todo!(),
            BuiltinIdentifier::Std => todo!(),
            BuiltinIdentifier::Core => todo!(),
            BuiltinIdentifier::Fp => todo!(),
            BuiltinIdentifier::Fn => todo!(),
            BuiltinIdentifier::FnMut => todo!(),
            BuiltinIdentifier::FnOnce => todo!(),
            BuiltinIdentifier::Array => todo!(),
            BuiltinIdentifier::Datasets => datasets::SCOPE_DATA,
            BuiltinIdentifier::DatasetType => todo!(),
            BuiltinIdentifier::Type => todo!(),
            BuiltinIdentifier::CloneTrait => todo!(),
            BuiltinIdentifier::CopyTrait => todo!(),
            BuiltinIdentifier::PartialEqTrait => todo!(),
            BuiltinIdentifier::EqTrait => todo!(),
        }
        .into(),
        EntityRouteKind::pack { main, .. } => EntitySource::Module { file: main },
        EntityRouteKind::ChildScope { parent, ident } => {
            this.subscope_table(parent)?.scope_source(ident)?
        }
        EntityRouteKind::Contextual { main, ident } => EntitySource::Contextual { main, ident },
        EntityRouteKind::Generic { ident, .. } => todo!(),
    })
}

pub struct ModuleFromFileError {
    pub rule_broken: ModuleFromFileRule,
}

pub enum ModuleFromFileRule {
    packNameShouldBeIdentifier,
    packRootShouldHaveFileName,
    FileShouldExist,
    FileShouldHaveExtensionHSK,
}

pub trait ScopeQueryGroup:
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
            Some(self.intern_scope(EntityRoute::child_scope(parent_scope, ident, generics)))
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
                    Ok(self.intern_scope(EntityRoute::pack(id, ident)))
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
                let _parent = self.module(self.alloc_file(path.with_file_name("mod.hsk")));
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
            EntitySource::Builtin(_) => panic!(),
            EntitySource::WithinModule { file: file_id, .. } => file_id,
            EntitySource::Module { file: file_id } => file_id,
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::Contextual { .. } => todo!(),
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

        module_path.map(|pth| self.alloc_file(pth))
    }

    fn raw_entity_kind_from_scope_kind(&self, scope_kind: &EntityRouteKind) -> RawEntityKind {
        raw_entity_kind_from_scope_kind(self.upcast(), scope_kind)
    }
}
