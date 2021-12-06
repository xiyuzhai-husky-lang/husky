//! Transforms markdown
use ide_db::helpers::rust_doc::is_rust_fence;

const RUSTDOC_FENCE: &str = "```";

pub(crate) fn format_docs(src: &str) -> String {
    let mut processed_lines = Vec::new();
    let mut in_code_block = false;
    let mut is_rust = false;

    for mut line in src.lines() {
        if in_code_block && is_rust && code_line_ignored_by_rustdoc(line) {
            continue;
        }

        if let Some(header) = line.strip_prefix(RUSTDOC_FENCE) {
            in_code_block ^= true;

            if in_code_block {
                is_rust = is_rust_fence(header);

                if is_rust {
                    line = "```rust";
                }
            }
        }

        if in_code_block {
            let trimmed = line.trim_start();
            if trimmed.starts_with("##") {
                line = &trimmed[1..];
            }
        }

        processed_lines.push(line);
    }
    processed_lines.join("\n")
}

fn code_line_ignored_by_rustdoc(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed == "#" || trimmed.starts_with("# ") || trimmed.starts_with("#\t")
}
