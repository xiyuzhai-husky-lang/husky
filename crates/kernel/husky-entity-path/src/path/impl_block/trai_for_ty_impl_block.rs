use super::*;

#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitForTypeImplBlockPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct TraitForTypeImplBlockPathData {
    module_path: ModulePath,
    trai_path: TraitPath,
    ty_sketch: TypeSketch,
    disambiguator: u8,
}

impl From<TraitForTypeImplBlockPath> for ItemPath {
    fn from(path: TraitForTypeImplBlockPath) -> Self {
        ItemPath::ImplBlock(path.into())
    }
}

impl TraitForTypeImplBlockPath {
    pub fn data(self, db: &::salsa::Db) -> TraitForTypeImplBlockPathData {
        match self.0.data(db) {
            ItemPathData::ImplBlock(ImplBlockPathData::TraitForTypeImplBlock(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.data(db).trai_path
    }

    pub fn ty_sketch(self, db: &::salsa::Db) -> TypeSketch {
        self.data(db).ty_sketch
    }

    pub(crate) fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TraitForTypeImplBlockPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TraitForTypeImplBlockPath {
        TraitForTypeImplBlockPath(id)
    }

    pub fn module_path(self) -> ModulePath {
        self.module_path
    }

    pub fn trai_path(self) -> TraitPath {
        self.trai_path
    }

    pub fn ty_sketch(self) -> TypeSketch {
        self.ty_sketch
    }

    pub fn disambiguator(self) -> u8 {
        self.disambiguator
    }

    pub(crate) fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty_sketch.show_aux(f, db)?;
        f.write_str(" as ")?;
        self.trai_path.show_aux(f, db)?;
        f.write_fmt(format_args!("({})", self.disambiguator))
    }
}

impl TraitForTypeImplBlockPath {
    pub fn new(
        db: &::salsa::Db,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        trai_path: TraitPath,
        ty_sketch: TypeSketch,
    ) -> Self {
        TraitForTypeImplBlockPath(ItemPathId::new(
            db,
            ItemPathData::ImplBlock(ImplBlockPathData::TraitForTypeImplBlock(
                TraitForTypeImplBlockPathData {
                    module_path,
                    trai_path,
                    ty_sketch,
                    disambiguator: registry.issue_disambiguitor(
                        module_path,
                        ImplBlockKind::TraitForType {
                            ty_sketch,
                            trai_path,
                        },
                    ),
                },
            )),
        ))
    }
}

impl salsa::DebugWithDb for TraitForTypeImplBlockPath {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str("TraitForTypeImplBlockPath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`)")
    }
}
