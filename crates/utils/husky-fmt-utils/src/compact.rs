pub struct FmtCompact<'a, T>(pub &'a T);

impl<'a, T> std::fmt::Debug for FmtCompact<'a, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.0))
    }
}
