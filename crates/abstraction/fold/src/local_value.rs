pub struct LocalValue<T: Copy> {
    values: Vec<(u8, T)>,
    current: u8,
}

impl<T> std::fmt::Debug for LocalValue<T>
where
    T: Copy + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\nLocalValue:\n"))?;
        f.write_fmt(format_args!("    values:\n"))?;
        for (version, value) in &self.values {
            f.write_fmt(format_args!("        {} {:?}\n", version, value))?;
        }
        Ok(())
    }
}

impl<T: Copy> LocalValue<T> {
    pub fn new(v: T) -> Self {
        Self {
            values: vec![(0, v)],
            current: 0,
        }
    }

    pub fn enter(&mut self) {
        self.current += 1;
    }

    pub fn exit(&mut self) {
        self.current -= 1;
        while self.values.last().unwrap().0 > self.current {
            self.values.pop();
        }
    }

    pub fn value(&self) -> T {
        self.values.last().unwrap().1
    }

    pub fn set(&mut self, v: T) {
        self.values.push((self.current, v))
    }
}

#[test]
fn use_fold_value() {
    let mut v: LocalValue<i64> = LocalValue::new(0);
    v.enter();
    v.set(22);
    v.set(23);
    v.enter();
    assert_eq!(v.value(), 23);
    v.exit();
    assert_eq!(v.value(), 23);
    v.exit();
    assert_eq!(v.value(), 0);
    assert_eq!(v.values.len(), 1);
}
