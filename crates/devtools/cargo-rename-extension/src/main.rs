use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// src
    #[clap(value_parser, name = "from")]
    from: String,
    /// dst
    #[clap(value_parser, name = "to")]
    to: String,
}

fn main() {
    let cli = Cli::parse();
    RenameExtensionInstance {
        from: &cli.from,
        to: &cli.to,
        dir: "./".into(),
    }
    .rename_all()
    .unwrap()
}

struct RenameExtensionInstance<'a> {
    from: &'a str,
    to: &'a str,
    dir: PathBuf,
}

impl<'a> RenameExtensionInstance<'a> {
    pub fn rename_all(self) -> std::io::Result<()> {
        for entry in std::fs::read_dir(&self.dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(extension) = extension.to_str() {
                        if extension == self.from {
                            let renamed_path = path.with_extension(self.to);
                            println!("{:?} -> {:?}?", path, renamed_path);
                            std::fs::rename(path, renamed_path)?
                        }
                    }
                }
            } else if path.is_dir() {
                self.subinstance(path).rename_all()?
            }
        }
        Ok(())
    }

    pub fn subinstance(&self, subdir: PathBuf) -> Self {
        Self {
            from: self.from,
            to: self.to,
            dir: subdir,
        }
    }
}
