#![allow(dead_code, unused)]
pub struct Callable {}
/// Primary API to get semantic information, like types, from syntax trees.
pub struct Semantics<'db, DB> {
    pub db: &'db DB,
    imp: SemanticsImpl<'db>,
}

pub struct SemanticsImpl<'db> {
    pub db: &'db dyn db::HirDatabase,
}
pub struct SemanticsScope<'a> {
    pub db: &'a dyn db::HirDatabase,
    // file_id: HirFileID,
    // resolver: Resolver,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct TypeParam {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct GenericDef {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Type {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Struct {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Union {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Enum {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Trait {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Field {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Variant {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Function {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Static {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Const {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct TypeAlias {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct AssocItem {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Crate {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ModuleSource {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct FieldSource {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ScopeDef {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct AssocItemContainer {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefixKind {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ModPath {}
pub struct HasSource {}
pub mod db {

    // FIXME: rename to ExpandDatabase
    #[salsa::query_group(AstDatabaseStorage)]
    pub trait AstDatabase: base_db::SourceDatabase {}

    #[salsa::query_group(InternDatabaseStorage)]
    pub trait InternDatabase: base_db::SourceDatabase {}

    #[salsa::query_group(DefDatabaseStorage)]
    pub trait DefDatabase: InternDatabase + AstDatabase + base_db::Upcast<dyn AstDatabase> {}

    #[salsa::query_group(HirDatabaseStorage)]
    pub trait HirDatabase: DefDatabase + base_db::Upcast<dyn DefDatabase> {}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Adt {}

pub trait AsAssocItem {
    fn as_assoc_item(self, db: &dyn db::HirDatabase) -> Option<AssocItem>;
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BuiltinType {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct GenericParam {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct HasVisibility {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Impl {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ItemInNs {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Label {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Local {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct MacroDef {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Module {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct ModuleDef {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Name {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct PathResolution {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Visibility {}
pub mod import_map {
    pub struct Query {}
}
pub struct InactiveCode {}
pub struct Namespace {}
pub struct AttrsWithOwner {}

pub struct AddReferenceHere {}
pub struct BreakOutsideOfLoop {}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Documentation {}
#[derive(Clone)]
pub enum Mutability {}
pub struct HirDisplay {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Param {}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct StructKind {}
pub struct Attr {}
pub struct IncorrectCase {}
pub struct InvalidDeriveTarget {}
pub struct MismatchedArgCount {}
pub struct MissingFields {}
pub struct MissingMatchArms {}
pub struct MissingOkOrSomeInTailExpr {}
pub struct NoSuchField {}
pub struct RemoveThisSemicolon {}
pub struct ReplaceFilterMapNextWithFindMap {}
pub struct UnresolvedExternPackage {}
pub struct UnresolvedModule {}
pub struct ConstParam {}
pub struct UnresolvedImport {}
pub struct SelfParam {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFile<T> {
    pub file_id: vfs::FileID,
    pub value: T,
}
