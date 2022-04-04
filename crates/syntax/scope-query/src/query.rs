use crate::{error::scope_err, *};
use check_utils::should;
use file::FilePtr;
use path_utils::*;
use scope::*;

use entity_syntax::RawTyKind;
use upcast::Upcast;
use word::{
    dash_to_snake, BuiltinIdentifier, ContextualIdentifier, CustomIdentifier, Identifier, WordPtr,
};

use fold::FoldStorage;

use std::{ops::Deref, path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryGroupStorage)]
pub trait ScopeSalsaQueryGroup: token::TokenQueryGroup + AllocateUniqueScope {
    fn subscope_table(&self, scope_id: ScopePtr) -> ScopeResultArc<SubscopeTable>;

    fn subscopes(&self, scope: ScopePtr) -> Arc<Vec<ScopePtr>>;

    fn raw_entity_kind(&self, scope_id: ScopePtr) -> RawEntityKind;

    fn scope_source(&self, scope_id: ScopePtr) -> ScopeResult<ScopeSource>;
}

fn subscope_table(
    this: &dyn ScopeSalsaQueryGroup,
    scope_id: ScopePtr,
) -> ScopeResultArc<SubscopeTable> {
    Ok(Arc::new(match this.scope_source(scope_id)? {
        ScopeSource::Builtin(data) => SubscopeTable::builtin(this, data),
        ScopeSource::WithinModule {
            file: file_id,
            token_group_index,
        } => {
            let text = this.tokenized_text(file_id)?;
            let item = text.fold_iter(token_group_index).next().unwrap();
            if let Some(children) = item.children {
                SubscopeTable::parse(file_id, children)
            } else {
                SubscopeTable::empty()
            }
        }
        ScopeSource::Module { file: file_id } => {
            let text = this.tokenized_text(file_id)?;
            SubscopeTable::parse(file_id, text.fold_iter(0))
        }
        ScopeSource::WithinBuiltinModule => todo!(),
        ScopeSource::Contextual { .. } => todo!(),
    }))
}

fn subscopes(db: &dyn ScopeSalsaQueryGroup, scope: ScopePtr) -> Arc<Vec<ScopePtr>> {
    Arc::new(db.subscope_table(scope).map_or(Vec::new(), |table| {
        table
            .subscopes(scope)
            .into_iter()
            .map(|scope| db.intern_scope(scope))
            .collect()
    }))
}

fn raw_entity_kind(db: &dyn ScopeSalsaQueryGroup, scope: ScopePtr) -> RawEntityKind {
    raw_entity_kind_from_scope_kind(db, &scope.kind)
}

fn raw_entity_kind_from_scope_kind(
    db: &dyn ScopeSalsaQueryGroup,
    scope_kind: &ScopeKind,
) -> RawEntityKind {
    match scope_kind {
        ScopeKind::Builtin { ident } => match ident {
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
        },
        ScopeKind::Package { .. } => RawEntityKind::Module,
        ScopeKind::ChildScope { parent, ident } => db
            .subscope_table(*parent)
            .unwrap()
            .raw_entity_kind(*ident)
            .unwrap(),
        ScopeKind::Contextual { ident, .. } => match ident {
            ContextualIdentifier::Input => RawEntityKind::Feature,
            ContextualIdentifier::ThisData => todo!(),
            ContextualIdentifier::ThisType => todo!(),
        },
        ScopeKind::Generic { ident } => todo!(),
    }
}

fn scope_source(this: &dyn ScopeSalsaQueryGroup, scope: ScopePtr) -> ScopeResult<ScopeSource> {
    Ok(match scope.kind {
        ScopeKind::Builtin { ident } => match ident {
            BuiltinIdentifier::Void => todo!(),
            BuiltinIdentifier::I32 => todo!(),
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => todo!(),
            BuiltinIdentifier::True => todo!(),
            BuiltinIdentifier::False => todo!(),
            BuiltinIdentifier::Vec => &BuiltinScopeData {
                subscopes: &[],
                signature: BuiltinScopeSignature::Vec,
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
        }
        .into(),
        ScopeKind::Package { main, .. } => ScopeSource::Module { file: main },
        ScopeKind::ChildScope { parent, ident } => {
            this.subscope_table(parent)?.scope_source(ident)?
        }
        ScopeKind::Contextual { main, ident } => ScopeSource::Contextual { main, ident },
        ScopeKind::Generic { ident } => todo!(),
    })
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

pub trait ScopeQueryGroup:
    ScopeSalsaQueryGroup + AllocateUniqueScope + Upcast<dyn ScopeSalsaQueryGroup>
{
    fn subscope(
        &self,
        parent_scope: ScopePtr,
        ident: CustomIdentifier,
        generics: Vec<GenericArgument>,
    ) -> Option<ScopePtr> {
        let parent_subscope_table = self.subscope_table(parent_scope);
        if parent_subscope_table.map_or(false, |table| table.has_subscope(ident, &generics)) {
            Some(self.intern_scope(Scope::child_scope(parent_scope, ident, generics)))
        } else {
            None
        }
    }

    fn all_modules(&self) -> Vec<ScopePtr>
    where
        Self: Sized,
    {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<ScopePtr>
    where
        Self: Sized,
    {
        self.all_modules().into_iter()
    }

    fn collect_modules(&self, id: FilePtr) -> Vec<ScopePtr>
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

    fn module(&self, id: FilePtr) -> ScopeResult<ScopePtr> {
        let path: PathBuf = (*id).into();
        if !self.file_exists(id) {
            scope_err!(format!("file didn't exist"))?
        } else if path_has_file_name(&path, "main.hsk") {
            if let Some(package_name) = path_parent_file_name_str(&path) {
                let snake_name = dash_to_snake(&package_name);
                if let WordPtr::Identifier(Identifier::Custom(ident)) =
                    self.word_allocator().alloc(snake_name)
                {
                    Ok(self.intern_scope(Scope::package(id, ident)))
                } else {
                    scope_err!(format!("package name should be identifier"))?
                }
            } else {
                scope_err!(format!("package root should have filename"))?
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

    fn module_file(&self, module: ScopePtr) -> ScopeResult<FilePtr> {
        Ok(match self.scope_source(module)? {
            ScopeSource::Builtin(_) => panic!(),
            ScopeSource::WithinModule { file: file_id, .. } => file_id,
            ScopeSource::Module { file: file_id } => file_id,
            ScopeSource::WithinBuiltinModule => todo!(),
            ScopeSource::Contextual { .. } => todo!(),
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

    fn raw_entity_kind_from_scope_kind(&self, scope_kind: &ScopeKind) -> RawEntityKind {
        raw_entity_kind_from_scope_kind(self.upcast(), scope_kind)
    }
}
