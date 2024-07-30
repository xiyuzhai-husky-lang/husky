pub struct FmtPunctuated<'a, T, F> {
    pub(crate) items: &'a [T],
    pub(crate) punctuation: &'a str,
    pub(crate) f: F,
}

impl<'a, T, F> FmtPunctuated<'a, T, F>
where
    F: Fn(&T, &mut std::fmt::Formatter) -> std::fmt::Result,
{
    pub fn new(items: &'a [T], punctuation: &'a str, f: F) -> Self {
        Self {
            items,
            punctuation,
            f,
        }
    }
}

impl<'a, T, F> std::fmt::Display for FmtPunctuated<'a, T, F>
where
    F: Fn(&T, &mut std::fmt::Formatter) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut start_flag = true;
        for item in self.items {
            if start_flag {
                start_flag = false;
            } else {
                f.write_str(self.punctuation)?;
            }
            (self.f)(item, f)?;
        }
        Ok(())
    }
}

#[test]
pub(crate) fn fmt_puntuated_works() {
    use std::fmt::Display;
    assert_eq!(
        FmtPunctuated::new(&(0..5).collect::<Vec<_>>(), ",", |i, f| i.fmt(f)).to_string(),
        "0,1,2,3,4"
    )
}
