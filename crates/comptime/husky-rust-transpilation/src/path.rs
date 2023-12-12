use super::*;
use either::*;
use husky_entity_path::{
    AssociatedItemPath, FugitivePath, MajorItemPath, PatternPath, PreludeIntTypePath,
    PreludeNumTypePath, PreludeTypePath, PrincipalEntityPath, TraitForTypeItemPath, TraitItemPath,
    TraitPath, TypeItemPath, TypePath, TypeSketch, TypeVariantPath,
};
use husky_ethereal_signature::signature::HasEtherealSignatureTemplate;
use husky_hir_ty::HirType;
use husky_linkage::instantiation::LinkageInstantiation;
use husky_vfs::{CrateKind, ModulePathData, PackagePathSource};

impl<E> TranspileToRustWith<E> for AssociatedItemPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            AssociatedItemPath::TypeItem(slf) => slf.transpile_to_rust(builder),
            AssociatedItemPath::TraitItem(slf) => slf.transpile_to_rust(builder),
            AssociatedItemPath::TraitForTypeItem(slf) => slf.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for TypeVariantPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        self.parent_ty_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for PrincipalEntityPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        match self {
            PrincipalEntityPath::Module(path) => path.ident(db).transpile_to_rust(builder),
            PrincipalEntityPath::MajorItem(path) => path.transpile_to_rust(builder),
            PrincipalEntityPath::TypeVariant(path) => {
                match path.parent_ty_path(db).prelude_ty_path(db) {
                    Some(PreludeTypePath::Option | PreludeTypePath::Result) => (),
                    _ => {
                        path.parent_ty_path(db).ident(db).transpile_to_rust(builder);
                        builder.punctuation(RustPunctuation::ColonColon);
                    }
                }
                path.ident(db).transpile_to_rust(builder)
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for MajorItemPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        match self {
            MajorItemPath::Type(slf) => slf.transpile_to_rust(builder),
            _ => self.ident(db).transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for FugitivePath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        self.module_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for TypePath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        match self.refine(db) {
            Left(PreludeTypePath::Num(PreludeNumTypePath::Int(
                path @ (PreludeIntTypePath::R8
                | PreludeIntTypePath::R16
                | PreludeIntTypePath::R32
                | PreludeIntTypePath::R64
                | PreludeIntTypePath::R128
                | PreludeIntTypePath::RSize),
            ))) => match path {
                PreludeIntTypePath::R8 => builder.r8(),
                PreludeIntTypePath::R16 => builder.r16(),
                PreludeIntTypePath::R32 => builder.r32(),
                PreludeIntTypePath::R64 => builder.r64(),
                PreludeIntTypePath::R128 => builder.r128(),
                PreludeIntTypePath::RSize => builder.rsize(),
                _ => unreachable!(),
            },
            Left(PreludeTypePath::UNIT) => builder.unit(),
            Left(PreludeTypePath::StringLiteral) => todo!(),
            Left(_) => self.ident(db).transpile_to_rust(builder),
            _ => {
                self.module_path(db).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::ColonColon);
                self.ident(db).transpile_to_rust(builder)
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for ModulePath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        match self.data(db) {
            ModulePathData::Root(crate_path) => {
                if Some(crate_path) == builder.crate_path {
                    builder.crate_()
                } else {
                    let package_path = crate_path.package_path(db);
                    match package_path.data(db) {
                        PackagePathSource::Library => match package_path.ident(db).data(db) {
                            "core" => builder.husky_core(),
                            _ => todo!(),
                        },
                        _ => crate_path.package_ident(db).transpile_to_rust(builder),
                    }
                }
            }
            ModulePathData::Child { parent, ident } => {
                parent.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::ColonColon);
                ident.transpile_to_rust(builder)
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for TraitPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for PatternPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            PatternPath::Type(path) => path.transpile_to_rust(builder),
            PatternPath::TypeVariant(path) => path.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for TypeItemPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.impl_block(db).ty_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for TraitItemPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.trai_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for TraitForTypeItemPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        builder.bracketed(RustBracket::Angle, |builder| {
            match self.impl_block(db).ty_sketch(db) {
                TypeSketch::DeriveAny => builder.todo(),
                TypeSketch::Path(path) => path.transpile_to_rust(builder),
            }
            builder.keyword(RustKeyword::As);
            self.impl_block(db).trai_path(db).transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}
