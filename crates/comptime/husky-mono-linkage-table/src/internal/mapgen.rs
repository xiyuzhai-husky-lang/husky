use super::*;

/// extract from library for safer type wrapping and more efficient lookup
pub(super) fn generate_map<Linkage: IsLinkage>(
    target_crate: CratePath,
    library: &MonoLibraryStorage,
    db: &dyn HirDepsDb,
) -> HashMap<HirLinkageKey, (HirLinkageDeps, Linkage)> {
    todo!()
}
