use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::path::Path;

pub mod extract;
pub mod parse;

pub struct Makefile(makefile_lossless::Makefile);

impl fmt::Display for Makefile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Makefile {
    type Target = makefile_lossless::Makefile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Makefile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Makefile {
    pub(crate) fn new() -> Self {
        Makefile(makefile_lossless::Makefile::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, expect_file};

    #[test]
    fn parse_makefile_example() {
        // Example Makefile content as a string
        let makefile_content = r#"
.PHONY: build test clean

build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
"#;

        // Parse the Makefile content
        let mf: Makefile = makefile_content.parse().unwrap();

        // Convert the Makefile back to a string
        let makefile_string = mf.to_string();

        // Use expect_test to compare the parsed Makefile string
        expect![[r#"

            .PHONY: build test clean

            build:
            	cargo build --release

            test:
            	cargo test

            clean:
            	cargo clean
        "#]]
        .assert_eq(&makefile_string);

        // Use expect_test to compare with the original content
        expect![[r#"
            .PHONY: build test clean

            build:
            	cargo build --release

            test:
            	cargo test

            clean:
            	cargo clean"#]]
        .assert_eq(makefile_content.trim());

        // Use expect_test to check the rules in the Makefile
        let rules_string = mf
            .rules()
            .map(|r| r.targets().collect::<Vec<String>>().join(" "))
            .collect::<Vec<_>>()
            .join(", ");

        expect![[r#".PHONY, build, test, clean"#]].assert_eq(&rules_string);
    }
}
