#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CyclicSlice<'a, T> {
    pub start: i32,
    pub end: i32,
    pub total: &'a [T],
}

impl<'eval, T> std::ops::Index<i32> for CyclicSlice<'eval, T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        &self.total[index.rem_euclid(self.total.len() as i32) as usize]
    }
}

impl<'eval, T> std::ops::Index<usize> for CyclicSlice<'eval, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.total[index.rem_euclid(self.total.len())]
    }
}

impl<'a, T> CyclicSlice<'a, T> {
    pub fn first(&self) -> Option<&T> {
        if self.total.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(&self[self.start])
        }
    }
    pub fn firstx(&self) -> &T {
        self.first().unwrap()
    }
    pub fn last(&self) -> Option<&T> {
        if self.total.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(&self[self.end - 1])
        }
    }
    pub fn lastx(&self) -> &T {
        self.last().unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (self.start..self.end).map(|i| &self[i])
    }

    pub fn enum_iter(&self) -> impl Iterator<Item = (i32, &T)> {
        (self.start..self.end).map(|i| (i, &self[i]))
    }
}
