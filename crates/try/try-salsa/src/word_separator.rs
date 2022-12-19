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
        let take = std::mem::take(&mut self.word_in_progress);
        match take {
            Some(/* this is an initialization */ word2) => self.completed_words.push(word2),
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
        split_by_whitespace("haha s"),
        ["haha".to_string(), "s".to_string()]
    )
}

fn split_by_whitespace(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    for c in s.chars() {
        if c.is_whitespace() {
            if !current_word.is_empty() {
                words.push(current_word);
                current_word = String::new();
            }
        } else {
            current_word.push(c);
        }
    }
    if !current_word.is_empty() {
        words.push(current_word);
    }
    words
}
