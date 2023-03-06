use husky_entity_path::*;
use husky_vfs::VfsPathMenu;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = TermPreludeDb, jar = TermJar)]
pub enum TermEntityPath {
    Form(FormPath),
    Trait(TraitPath),
    TypeOntology(TypePath),
    TypeConstructor(TypePath),
}

impl TermEntityPath {
    pub fn ty_ontology(self) -> Option<TypePath> {
        match self {
            TermEntityPath::TypeOntology(path) => Some(path),
            TermEntityPath::Form(_)
            | TermEntityPath::Trait(_)
            | TermEntityPath::TypeConstructor(_) => None,
        }
    }
}

impl From<FormPath> for TermEntityPath {
    fn from(value: FormPath) -> Self {
        TermEntityPath::Form(value)
    }
}

impl From<TraitPath> for TermEntityPath {
    fn from(value: TraitPath) -> Self {
        TermEntityPath::Trait(value)
    }
}

impl TermEntityPath {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermPreludeDb,
    ) -> std::fmt::Result {
        // .display_with_db_fmt(f, db, salsa::DisplayFormatLevel::root())
        todo!()
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// #[enum_class::from_variants]
// #[salsa::derive_debug_with_db(db = TermPreludeDb, jar = TermPreludeJar)]
// pub enum TermTypePath {
//     Prelude(PreludeTypePath),
//     Custom(CustomTypePath),
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct CustomTypePath(TypePath);

// #[salsa::tracked(jar = TermPreludeJar)]
// pub(crate) fn term_ty_path(
//     db: &dyn TermPreludeDb,
//     path: TypePath,
// ) -> TermPreludeResult<TermTypePath> {
//     TermTypePath::from_ty_path(db, path)
// }

// impl TermTypePath {
//     fn from_ty_path(db: &dyn TermPreludeDb, path: TypePath) -> TermPreludeResult<Self> {
//         let menu: &EntityPathMenu = db.entity_path_menu(path.toolchain(db))?;
//         let vfs_path_menu: &VfsPathMenu = db.vfs_path_menu(path.toolchain(db))?;
//         if path.crate_path(db) != vfs_path_menu.core_library() {
//             return Ok(CustomTypePath(path).into());
//         }
//         Ok(match path {
//             path if path == menu.unit_ty_path() => PreludeTypePath::Unit,
//             path if path == menu.never_ty_path() => PreludeTypePath::Never,
//             path if path == menu.bool_ty_path() => PreludeTypePath::Bool,
//             path if path == menu.i8_ty_path() => PreludeTypePath::I8,
//             path if path == menu.i16_ty_path() => PreludeTypePath::I16,
//             path if path == menu.i32_ty_path() => PreludeTypePath::I32,
//             path if path == menu.i64_ty_path() => PreludeTypePath::I64,
//             path if path == menu.i128_ty_path() => PreludeTypePath::I128,
//             path if path == menu.isize_ty_path() => PreludeTypePath::ISize,
//             path if path == menu.u8_ty_path() => PreludeTypePath::U8,
//             path if path == menu.u16_ty_path() => PreludeTypePath::U16,
//             path if path == menu.u32_ty_path() => PreludeTypePath::U32,
//             path if path == menu.u64_ty_path() => PreludeTypePath::U64,
//             path if path == menu.u128_ty_path() => PreludeTypePath::U128,
//             path if path == menu.usize_ty_path() => PreludeTypePath::USize,
//             path if path == menu.r8_ty_path() => PreludeTypePath::R8,
//             path if path == menu.r16_ty_path() => PreludeTypePath::R16,
//             path if path == menu.r32_ty_path() => PreludeTypePath::R32,
//             path if path == menu.r64_ty_path() => PreludeTypePath::R64,
//             path if path == menu.r128_ty_path() => PreludeTypePath::R128,
//             path if path == menu.rsize_ty_path() => PreludeTypePath::RSize,
//             path if path == menu.f32_ty_path() => PreludeTypePath::F32,
//             path if path == menu.f64_ty_path() => PreludeTypePath::F64,
//             path if path == menu.lifetime_ty_path() => PreludeTypePath::Lifetime,
//             path if path == menu.module_ty_path() => PreludeTypePath::Module,
//             path if path == menu.trai_ty_path() => PreludeTypePath::Trait,
//             path if path == menu.ref_ty_path() => PreludeTypePath::Ref,
//             path if path == menu.ref_mut_ty_path() => PreludeTypePath::RefMut,
//             path if path == menu.list_ty_path() => PreludeTypePath::List,
//             path if path == menu.slice_ty_path() => PreludeTypePath::Slice,
//             path if path == menu.string_literal_ty_path() => PreludeTypePath::StringLiteral,
//             path if path == menu.str_ty_path() => PreludeTypePath::Str,
//             _ => return Ok(CustomTypePath(path).into()),
//         }
//         .into())
//     }
// }
