#[derive(PartialEq, Eq)]
pub(crate) enum ExpectTestConfig {
    UpdateExpect,
    KeepExpect,
}

impl ExpectTestConfig {
    pub(crate) fn from_env() -> Self {
        match std::env::var("UPDATE_EXPECT") {
            Ok(v) => match v.as_str() {
                "1" => ExpectTestConfig::UpdateExpect,
                _ => todo!(),
            },
            Err(_) => ExpectTestConfig::KeepExpect,
        }
    }
}
