use husky_eval::Session;

pub enum HuskyRuntimeVariant {
    None,
    Learning { session: Session<'static> },
}
