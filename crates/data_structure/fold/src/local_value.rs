#[derive(Debug)]
pub struct LocalValue<T: Copy> {
    values: Vec<(u8, T)>,
    current: u8,
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
        if self.values.last().unwrap().0 > self.current {
            self.values.pop();
        }
    }

    pub fn value(&self) -> T {
        self.values.last().unwrap().1
    }

    pub fn set_value(&mut self, v: T) {
        self.values.push((self.current, v))
    }
}

#[test]
fn use_fold_value() {
    let mut v: LocalValue<i64> = LocalValue::new(0);
    v.enter();
    v.set_value(23);
    v.enter();
    assert_eq!(v.value(), 23);
    v.exit();
    assert_eq!(v.value(), 23);
    v.exit();
    assert_eq!(v.value(), 0);
    assert_eq!(v.values.len(), 1);
}
