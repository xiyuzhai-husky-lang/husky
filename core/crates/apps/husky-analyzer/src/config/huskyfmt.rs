#[derive(Debug, Clone)]
pub enum HuskyfmtConfig {
    Huskyfmt {
        extra_args: Vec<String>,
        enable_range_formatting: bool,
    },
    CustomCommand {
        command: String,
        args: Vec<String>,
    },
}
