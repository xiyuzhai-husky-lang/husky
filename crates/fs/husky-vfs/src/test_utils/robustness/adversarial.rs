use husky_adversarial_utils::{new_rand_string, new_rand_string2};

use super::*;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(super) enum VfsAdversarial {
    InsertNewLine { position: usize },
    InsertSpace { position: usize, nspace: u8 },
    InsertString { position: usize, insertion: String },
    DeleteToEof { position: usize },
    DeleteToEol { position: usize, delete_eol: bool },
}

impl VfsAdversarial {
    const NUMBER_OF_VARIANTS: u8 = 4;

    pub(super) fn new_rand(text: &str, rng: &mut XRng) -> Self {
        let kind: u8 = rng.randint(0, VfsAdversarial::NUMBER_OF_VARIANTS);
        match kind {
            0 => Self::new_rand_insert_new_line(text, rng),
            1 => Self::new_rand_insert_space(text, rng),
            2 => Self::new_rand_insert_string(text, rng),
            3 => Self::new_rand_delete_to_eof(text, rng),
            4 => Self::new_rand_delete_to_eol(text, rng),
            _ => unreachable!(),
        }
    }

    pub(super) fn edit(&self, text: &str) -> String {
        self.to_edit(text).apply(text)
    }

    fn to_edit(&self, text: &str) -> VfsEdit {
        match self {
            VfsAdversarial::InsertNewLine { position } => VfsEdit::InsertString {
                position: *position,
                insertion: "\n".into(),
            },
            VfsAdversarial::InsertSpace { position, nspace } => VfsEdit::InsertString {
                position: *position,
                insertion: (0..*nspace)
                    .into_iter()
                    .map(|_| ' ')
                    .collect::<String>()
                    .into(),
            },
            VfsAdversarial::InsertString {
                position,
                insertion,
            } => VfsEdit::InsertString {
                position: *position,
                insertion: insertion.into(),
            },
            VfsAdversarial::DeleteToEof { position } => VfsEdit::DeleteToEof {
                position: *position,
            },
            VfsAdversarial::DeleteToEol {
                position,
                delete_eol,
            } => VfsEdit::DeleteToEol {
                position: *position,
                delete_eol: *delete_eol,
            },
        }
    }

    fn new_rand_insert_new_line(text: &str, rng: &mut XRng) -> VfsAdversarial {
        VfsAdversarial::InsertNewLine {
            position: new_rand_position(text, rng),
        }
    }

    fn new_rand_insert_space(text: &str, rng: &mut XRng) -> VfsAdversarial {
        VfsAdversarial::InsertSpace {
            position: new_rand_position(text, rng),
            nspace: rng.randint(0, 5),
        }
    }

    fn new_rand_insert_string(text: &str, rng: &mut XRng) -> VfsAdversarial {
        static pieces: &'static [&'static str] = &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "$", "%",
        ];
        VfsAdversarial::InsertString {
            position: new_rand_position(text, rng),
            insertion: new_rand_string2(rng, 5, pieces),
        }
    }

    fn new_rand_delete_to_eof(text: &str, rng: &mut XRng) -> VfsAdversarial {
        VfsAdversarial::DeleteToEof {
            position: new_rand_position(text, rng),
        }
    }

    fn new_rand_delete_to_eol(text: &str, rng: &mut XRng) -> VfsAdversarial {
        VfsAdversarial::DeleteToEol {
            position: new_rand_position(text, rng),
            delete_eol: rng.randbool(),
        }
    }
}

fn new_rand_position(text: &str, rng: &mut XRng) -> usize {
    rng.randint(0, text.chars().count())
}
