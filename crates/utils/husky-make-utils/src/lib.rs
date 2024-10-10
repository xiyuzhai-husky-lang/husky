pub use makefile_lossless::Makefile;

use makefile_lossless::*;
use std::fs::File;

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

    println!("Makefile as string:\n{}", makefile_string);

    // You can also compare it with the original content
    assert_eq!(makefile_string.trim(), makefile_content.trim());

    // Print the rules in the Makefile
    println!(
        "Rules in the makefile: {:?}",
        mf.rules()
            .map(|r| r.targets().collect::<Vec<String>>().join(" "))
            .collect::<Vec<_>>()
    );

    // Additional assertions to verify the parsed content
    let rules: Vec<_> = mf.rules().collect();
    assert_eq!(rules.len(), 3);
    assert_eq!(rules[0].targets().next().unwrap(), "build");
    assert_eq!(rules[1].targets().next().unwrap(), "test");
    assert_eq!(rules[2].targets().next().unwrap(), "clean");
}
