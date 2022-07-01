use husky_feature_eval::Session;

pub enum HuskyEvalTimeVariant {
    None,
    Learning { session: Session<'static> },
}
