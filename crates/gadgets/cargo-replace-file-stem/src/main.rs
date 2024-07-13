use clap::Parser;
use std::{ffi::OsString, path::PathBuf, str::FromStr};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// cargo command name
    #[clap(value_parser, name = "cargo_command_name")]
    cargo_command_name: String,
    /// src
    #[clap(value_parser, name = "from")]
    from: String,
    /// dst
    #[clap(value_parser, name = "to")]
    to: String,
}

fn main() {
    let cli = Cli::parse();
    let instance = ReplaceFileStemInstance {
        from: &cli.from,
        to: &cli.to,
    };
    println!("from = {}, to = {}", &instance.from, &instance.to);
    instance.replace_all("./".into()).unwrap()
}

struct ReplaceFileStemInstance<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> ReplaceFileStemInstance<'a> {
    pub fn replace_all(&self, dir: PathBuf) -> std::io::Result<()> {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let Some(file_stem) = path.file_stem().expect("to be some").to_str() else {
                continue;
            };
            let new_path = if file_stem == self.from {
                use husky_cli_utils::ask::ask_user_for_permission;

                let mut file_name = OsString::from_str(self.to).unwrap();
                match path.extension() {
                    Some(extension) => {
                        file_name.push(".");
                        file_name.push(extension);
                    }
                    None => (),
                };
                let renamed_path = path.with_file_name(file_name);
                //  path.with_extension(self.to);
                if ask_user_for_permission(format!(
                    "Shall {:?} be renamed to {:?}?",
                    path, renamed_path
                )) {
                    std::fs::rename(path, renamed_path.clone())?;
                    renamed_path
                } else {
                    path
                }
            } else {
                path
            };
            if new_path.is_dir() {
                self.replace_all(new_path)?
            }
        }
        Ok(())
    }
}
