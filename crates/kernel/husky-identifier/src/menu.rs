use crate::*;

pub struct WordMenu {
    i32: WordItd,
    i64: WordItd,
    b32: WordItd,
    b64: WordItd,
    f32: WordItd,
    f64: WordItd,
}

impl WordMenu {
    pub fn use_(&self) -> WordItd {
        Keyword::Use.into()
    }
    pub fn impl_(&self) -> WordItd {
        todo!()
    }

    pub fn visual(&self) -> WordItd {
        Keyword::Visual.into()
    }

    pub fn mod_(&self) -> WordItd {
        Keyword::Mod.into()
    }

    pub fn i32(&self) -> WordItd {
        self.i32
    }

    pub fn i64(&self) -> WordItd {
        self.i64
    }

    pub fn b32(&self) -> WordItd {
        self.b32
    }

    pub fn b64(&self) -> WordItd {
        self.b64
    }

    pub fn f32(&self) -> WordItd {
        self.f32
    }

    pub fn f64(&self) -> WordItd {
        self.f64
    }
}
