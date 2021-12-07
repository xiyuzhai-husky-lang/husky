#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LineEndingType {
    Unix,
    Dos,
}

pub struct FileData {
    pub(super) content: String,
    pub(super) line_ending_type: LineEndingType,
}

impl FileData {
    pub(super) fn new(mut bytes: Vec<u8>) -> FileData {
        if !bytes.contains(&b'\r') {
            return FileData {
                content: String::from_utf8(bytes).expect(""),
                line_ending_type: LineEndingType::Unix,
            };
        }

        // We replace `\r\n` with `\n` in-place, which doesn't break utf-8 encoding.
        // While we *can* call `as_mut_vec` and do surgery on the live string
        // directly, let's rather steal the contents of `src`. This makes the code
        // safe even if a panic occurs.

        let mut gap_len = 0;
        let mut tail = bytes.as_mut_slice();
        loop {
            let idx = match find_crlf(&tail[gap_len..]) {
                None => tail.len(),
                Some(idx) => idx + gap_len,
            };
            tail.copy_within(gap_len..idx, 0);
            tail = &mut tail[idx - gap_len..];
            if tail.len() == gap_len {
                break;
            }
            gap_len += 1;
        }

        // Account for removed `\r`.
        // After `set_len`, `buf` is guaranteed to contain utf-8 again.
        let new_len = bytes.len() - gap_len;
        let src = unsafe {
            bytes.set_len(new_len);
            String::from_utf8_unchecked(bytes)
        };
        return FileData {
            content: src,
            line_ending_type: LineEndingType::Dos,
        };

        fn find_crlf(src: &[u8]) -> Option<usize> {
            src.windows(2).position(|it| it == b"\r\n")
        }
    }
}
