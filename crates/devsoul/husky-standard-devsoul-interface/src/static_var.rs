use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StandardStaticVarId {
    data: [u64; 4],
}

impl StandardStaticVarId {
    pub fn new(data: [u64; 4]) -> Self {
        Self { data }
    }
}

impl From<u32> for StandardStaticVarId {
    fn from(data: u32) -> Self {
        Self {
            data: [data as u64, 0, 0, 0],
        }
    }
}

impl Into<u32> for StandardStaticVarId {
    fn into(self) -> u32 {
        self.data[0] as u32
    }
}

impl From<u64> for StandardStaticVarId {
    fn from(data: u64) -> Self {
        Self {
            data: [data, 0, 0, 0],
        }
    }
}

impl Into<u64> for StandardStaticVarId {
    fn into(self) -> u64 {
        self.data[0]
    }
}

impl From<usize> for StandardStaticVarId {
    fn from(data: usize) -> Self {
        Self {
            data: [data as u64, 0, 0, 0],
        }
    }
}

impl Into<usize> for StandardStaticVarId {
    fn into(self) -> usize {
        self.data[0] as usize
    }
}

impl From<[u64; 4]> for StandardStaticVarId {
    fn from(data: [u64; 4]) -> Self {
        Self { data }
    }
}
