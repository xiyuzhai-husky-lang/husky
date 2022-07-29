use super::*;
use husky_pattern_syntax::RawPattern;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub fn parse_pattern(&mut self) -> AtomResult<(RawPattern, Vec<HuskyAtom>)> {
        let pattern_parser = MatchPatternParser::new(self.parse_all_remaining_atoms()?);
        todo!();
        // while let Some(next_atom) = remaining_atoms {
        //     match next_atom.variant {
        //         AtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::BitOr)) => {
        //             if let Some(new_pattern) = self.parse_simple_pattern()? {
        //                 pattern = pattern.or(new_pattern)
        //             } else {
        //                 return err!("expect pattern after `|`", next_atom.range);
        //             }
        //         }
        //         _ => todo!(),
        //     }
        // }
        pattern_parser.finish()
    }
}

pub struct MatchPatternParser {
    atoms: Vec<HuskyAtom>,
    next: usize,
}

enum PatternOpr {
    Or,
}

impl MatchPatternParser {
    pub fn new(atoms: Vec<HuskyAtom>) -> Self {
        Self { atoms, next: 0 }
    }

    pub fn parse_pattern(&mut self) -> AtomResult<RawPattern> {
        let mut pattern = self.parse_simple_pattern()?.unwrap();
        while let Some((next_pattern_opr, range)) = self.next_pattern_opr() {
            match next_pattern_opr {
                PatternOpr::Or => {
                    if let Some(new_pattern) = self.parse_simple_pattern()? {
                        pattern = pattern.or(new_pattern)
                    } else {
                        return err!("expect pattern after `|`", range);
                    }
                }
                _ => todo!(),
            }
        }
        Ok(pattern)
    }

    fn parse_simple_pattern(&mut self) -> AtomResult<Option<RawPattern>> {
        Ok(if let Some(atom) = self.next_atom() {
            Some(match atom.variant {
                AtomVariant::EntityRoute { route, kind } => match kind {
                    EntityKind::EnumLiteral => RawPattern::enum_literal(route, atom.range),
                    _ => err!(format!("expect enum literal"), atom.range)?,
                },
                AtomVariant::PrimitiveLiteral(value) => {
                    RawPattern::primitive_literal(value, atom.range)
                }
                _ => err!(format!("expect pattern"), atom.range)?,
            })
        } else {
            None
        })
    }

    fn next_pattern_opr(&self) -> Option<(PatternOpr, TextRange)> {
        let atom = &self.next_atom()?;
        let kind = match atom.variant {
            AtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::BitOr)) => PatternOpr::Or,
            _ => todo!(),
        };
        Some((kind, atom.range))
    }

    fn next_atom(&self) -> Option<&HuskyAtom> {
        if self.next >= self.atoms.len() {
            None
        } else {
            Some(&self.atoms[self.next])
        }
    }

    fn finish(mut self) -> AtomResult<(RawPattern, Vec<HuskyAtom>)> {
        Ok((
            self.parse_pattern()?,
            self.atoms.drain(self.next..).collect(),
        ))
    }
}
