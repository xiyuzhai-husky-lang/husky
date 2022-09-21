use husky_feature_eval::Session;

pub enum HuskyRuntimeVariant {
    None,
    Learning { session: Session<'static> },
}
