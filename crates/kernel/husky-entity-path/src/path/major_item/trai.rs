use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
pub struct TraitPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitPathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
}

impl TraitPath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::MajorItem(MajorItemPathData::Trait(TraitPathData {
                module_path,
                ident,
                connection,
            })),
        ))
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db) -> String {
        self.display(db).to_string()
    }

    pub fn data(self, db: &::salsa::Db) -> TraitPathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Trait(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TraitPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TraitPath {
        TraitPath(id)
    }

    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn connection(&self) -> MajorItemConnection {
        self.connection
    }
}

impl salsa::DebugWithDb for TraitPath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str("TraitPath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for TraitPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}

#[enum_class::from_variants]
#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreludeTraitPath {
    Add,
    AddAssign,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Div,
    DivAssign,
    IntIndex,
    Mul,
    MulAssign,
    Neg,
    Not,
    Unveil,
    Clone,
    Copy,
    Default,
    Visualize,
}

impl PreludeTraitPath {
    pub const ADD: Self = PreludeTraitPath::Add;
    pub const ADD_ASSIGN: Self = PreludeTraitPath::AddAssign;
    pub const BIT_AND: Self = PreludeTraitPath::BitAnd;
    pub const BIT_AND_ASSIGN: Self = PreludeTraitPath::BitAndAssign;
    pub const BIT_OR: Self = PreludeTraitPath::BitOr;
    pub const BIT_OR_ASSIGN: Self = PreludeTraitPath::BitOrAssign;
    pub const BIT_XOR: Self = PreludeTraitPath::BitXor;
    pub const BIT_XOR_ASSIGN: Self = PreludeTraitPath::BitXorAssign;
    pub const DIV: Self = PreludeTraitPath::Div;
    pub const DIV_ASSIGN: Self = PreludeTraitPath::DivAssign;
    pub const INT_INDEX: Self = PreludeTraitPath::IntIndex;
    pub const MUL: Self = PreludeTraitPath::Mul;
    pub const MUL_ASSIGN: Self = PreludeTraitPath::MulAssign;
    pub const NEG: Self = PreludeTraitPath::Neg;
    pub const NOT: Self = PreludeTraitPath::Not;
    pub const UNVEIL: Self = PreludeTraitPath::Unveil;
    pub const CLONE: Self = PreludeTraitPath::Clone;
    pub const COPY: Self = PreludeTraitPath::Copy;
    pub const DEFAULT: Self = PreludeTraitPath::Default;
    pub const VISUALIZE: Self = PreludeTraitPath::Visualize;
}

impl TraitPath {
    pub fn prelude_trai_path(self, db: &::salsa::Db) -> Option<PreludeTraitPath> {
        prelude_trai_path(db, self)
    }
}

#[salsa::tracked(jar = EntityPathJar)]
fn prelude_trai_path(db: &::salsa::Db, path: TraitPath) -> Option<PreludeTraitPath> {
    let menu: &ItemPathMenu = item_path_menu(db, path.toolchain(db));
    let vfs_path_menu: &VfsPathMenu = db.vfs_path_menu(path.toolchain(db));
    if path.crate_path(db) != vfs_path_menu.core_library() {
        return None;
    }
    Some(match path {
        path if path == menu.add_trai_path() => PreludeTraitPath::ADD,
        path if path == menu.add_assign_trai_path() => PreludeTraitPath::ADD_ASSIGN,
        path if path == menu.bit_and_trai_path() => PreludeTraitPath::BIT_AND,
        path if path == menu.bit_and_assign_trai_path() => PreludeTraitPath::BIT_AND_ASSIGN,
        path if path == menu.bit_or_trai_path() => PreludeTraitPath::BIT_OR,
        path if path == menu.bit_or_assign_trai_path() => PreludeTraitPath::BIT_OR_ASSIGN,
        path if path == menu.bit_xor_trai_path() => PreludeTraitPath::BIT_XOR,
        path if path == menu.bit_xor_assign_trai_path() => PreludeTraitPath::BIT_XOR_ASSIGN,
        path if path == menu.div_trai_path() => PreludeTraitPath::DIV,
        path if path == menu.div_assign_trai_path() => PreludeTraitPath::DIV_ASSIGN,
        path if path == menu.int_index_trai_path() => PreludeTraitPath::INT_INDEX,
        path if path == menu.mul_trai_path() => PreludeTraitPath::MUL,
        path if path == menu.mul_assign_trai_path() => PreludeTraitPath::MUL_ASSIGN,
        path if path == menu.neg_trai_path() => PreludeTraitPath::NEG,
        path if path == menu.not_trai_path() => PreludeTraitPath::NOT,
        path if path == menu.unveil_trai_path() => PreludeTraitPath::UNVEIL,
        path if path == menu.clone_trai_path() => PreludeTraitPath::CLONE,
        path if path == menu.copy_trai_path() => PreludeTraitPath::COPY,
        path if path == menu.default_trai_path() => PreludeTraitPath::DEFAULT,
        path if path == menu.visualize_trai_path() => PreludeTraitPath::VISUALIZE,
        _ => return None,
    })
}

#[salsa::debug_with_db]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomTraitPath(TraitPath);

impl From<CustomTraitPath> for TraitPath {
    fn from(path: CustomTraitPath) -> Self {
        path.0
    }
}

impl TraitPath {
    pub fn refine(self, db: &::salsa::Db) -> Either<PreludeTraitPath, CustomTraitPath> {
        match self.prelude_trai_path(db) {
            Some(path) => Left(path),
            None => Right(CustomTraitPath(self)),
        }
    }
}
