use eval_feature::Session;

pub enum HuskyEvalTimeVariant {
    None,
    Learning { session: Session<'static> },
}
