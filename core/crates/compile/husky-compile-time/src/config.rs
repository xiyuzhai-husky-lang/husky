use husky_linkage_table::LinkageTableConfig;

#[derive(Debug, Clone)]
pub struct HuskyCompileTimeConfig {
    pub __resolve_root_defn:
        fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn,
    pub linkage_table: LinkageTableConfig,
}
