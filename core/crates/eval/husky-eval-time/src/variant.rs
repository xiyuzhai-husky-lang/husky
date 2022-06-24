use eval_feature::Session;

pub enum HuskyEvalTimeVariant {
    Boot,
    Learning { session: Session<'static> },
}
