#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CyclicSlice<'a, T> {
    pub start: i32,
    pub end: i32,
    pub total: &'a [T],
}

impl<'a, T> CyclicSlice<'a, T> {
    pub fn first(&self) -> Option<&T> {
        if self.total.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(&self.total[self.start.rem_euclid(self.total.len() as i32) as usize])
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
            Some(&self.total[(self.end - 1).rem_euclid(self.total.len() as i32) as usize])
        }
    }
    pub fn lastx(&self) -> &T {
        self.last().unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (self.start..self.end).map(|i| &self.total[i.rem_euclid(self.total.len() as i32) as usize])
    }

    pub fn enum_iter(&self) -> impl Iterator<Item = (i32, &T)> {
        (self.start..self.end).map(|i| {
            (
                i,
                &self.total[i.rem_euclid(self.total.len() as i32) as usize],
            )
        })
    }
}
