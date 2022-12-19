use std::str::Chars;

struct WordSplitter<'a> {
    completed_words: Vec<String>,
    word_in_progress: Option<String>,
    iter: Chars<'a>,
}

impl<'a> WordSplitter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            completed_words: vec![],
            word_in_progress: None,
            iter: s.chars(),
        }
    }
    fn step(&mut self, c: char) {
        match c {
            // meet whitespace
            ' ' => self.complete_current(),
            c => {
                if c.is_alphabetic() {
                    match &mut self.word_in_progress {
                        Some(current) => current.push(c),
                        None => self.word_in_progress = Some(c.into()),
                    }
                } else {
                    todo!()
                }
            }
        }
    }

    fn complete_current(&mut self) {
        match std::mem::take(&mut self.word_in_progress) {
            Some(word) => self.completed_words.push(word),
            None => (),
        }
    }

    fn parse_all(mut self) -> Vec<String> {
        while let Some(c) = self.iter.next() {
            self.step(c)
        }
        self.complete_current();
        self.completed_words
    }
}

#[test]

fn test_it() {
    assert_eq!(
        WordSplitter::new("haha s").parse_all(),
        ["haha".to_string(), "s".to_string()]
    )
}
