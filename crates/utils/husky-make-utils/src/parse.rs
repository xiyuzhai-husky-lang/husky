use crate::*;
use std::str::FromStr;

impl FromStr for Makefile {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check for proper indentation
        check_makefile_indentation(&s)?;

        // Parse the Makefile
        let makefile: makefile_lossless::Makefile = s.parse()?;
        Ok(Self(makefile))
    }
}

/// Checks if the given Makefile content contains any lines starting with spaces.
///
/// # Arguments
///
/// * `content` - A string slice containing the Makefile content.
///
/// # Returns
///
/// `Ok(())` if no lines start with spaces, otherwise an `Err` with a descriptive message.
fn check_makefile_indentation(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if content.lines().any(|line| line.starts_with(' ')) {
        Err("Makefile contains lines starting with spaces. Use tabs for indentation.".into())
    } else {
        Ok(())
    }
}
