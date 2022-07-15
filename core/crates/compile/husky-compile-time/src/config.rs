use husky_linkage_table::LinkageTableConfig;

#[derive(Debug, Clone)]
pub struct HuskyCompileTimeConfig {
    pub __root_defn_resolver:
        fn(ident: word::RootIdentifier) -> &'static static_defn::EntityStaticDefn,
    pub linkage_table: LinkageTableConfig,
}
