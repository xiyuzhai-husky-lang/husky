use crate::*;

pub(crate) struct CTranspiler<'a> {
    db: &'a dyn CHirDb,
    current_line_buffer: String,
}

impl<'a> CTranspiler<'a> {
    pub(crate) fn transpile_subhir(&mut self, ctx: &CTranspilerContext, hir: &impl CHirTranspile) {
        let subhir_ctx = ctx.subcontext(hir);
        hir.transpile(self, subhir_ctx)
    }

    pub(crate) fn c_expr_hir_arena(&self) -> &'a CExprHirArena {
        todo!()
    }

    pub(crate) fn transpile_comma_separated_subhirs<T: CHirTranspile>(
        &mut self,
        _ctx: &CTranspilerContext,
        _bracket: CBracket,
        _items: impl IntoIterator<Item = T>,
    ) {
        todo!()
    }
}

pub(crate) enum CBracket {
    Parenthesis,
    Curly,
}

pub(crate) struct CTranspilerContext {
    multiline: bool,
    pressure: u8,
}

impl CTranspilerContext {
    fn subcontext(&self, hir: &impl CHirTranspile) -> Self {
        let resistance = hir.resistance();
        let multiline = self.pressure > resistance;
        let pressure = if self.pressure > resistance {
            self.pressure - resistance
        } else {
            0
        };
        Self {
            multiline,
            pressure,
        }
    }

    pub(crate) fn multiline(&self) -> bool {
        self.multiline
    }
}

pub(crate) trait CHirTranspile {
    fn resistance(&self) -> u8;

    fn transpile(&self, transpiler: &mut CTranspiler, ctx: CTranspilerContext);
}

impl<T> CHirTranspile for &T
where
    T: CHirTranspile,
{
    fn resistance(&self) -> u8 {
        <T as CHirTranspile>::resistance(self)
    }

    fn transpile(&self, transpiler: &mut CTranspiler, ctx: CTranspilerContext) {
        <T as CHirTranspile>::transpile(self, transpiler, ctx)
    }
}
