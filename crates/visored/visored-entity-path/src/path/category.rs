#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdCategoryPath {
    /// the category of sets
    Set,
    /// the category of propositions
    Prop,
}

impl VdCategoryPath {
    pub const SET: Self = VdCategoryPath::Set;
    pub const PROPOSITION: Self = VdCategoryPath::Prop;
}

impl salsa::DisplayWithDb for VdCategoryPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        match self {
            VdCategoryPath::Set => write!(f, "Set"),
            VdCategoryPath::Prop => write!(f, "Prop"),
        }
    }
}
