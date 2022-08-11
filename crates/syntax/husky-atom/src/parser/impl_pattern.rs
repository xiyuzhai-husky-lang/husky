use super::*;
use husky_opn_syntax::{BinaryOpr, PureBinaryOpr};
use husky_pattern_syntax::RawPattern;

impl<'a, 'b> AtomParser<'a, 'b> {
    pub fn parse_pattern(&mut self) -> AtomResult<(RawPattern, Vec<HuskyAtom>)> {
        let pattern_parser = MatchPatternParser::new(self.parse_all_remaining_atoms()?);
        pattern_parser.parse_into_pattern()
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
        let mut pattern = self.parse_next_pattern()?.unwrap();
        while let Some((next_pattern_opr, range)) = self.next_pattern_opr() {
            match next_pattern_opr {
                PatternOpr::Or => {
                    if let Some(new_pattern) = self.parse_next_pattern()? {
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

    fn parse_next_pattern(&mut self) -> AtomResult<Option<RawPattern>> {
        Ok(if let Some(atom) = self.next_atom() {
            Some(match atom.variant {
                HuskyAtomVariant::EntityRoute { route, kind } => match kind {
                    EntityKind::EnumVariant => RawPattern::enum_literal(route, atom.range),
                    _ => err!(format!("expect enum literal"), atom.range)?,
                },
                HuskyAtomVariant::PrimitiveLiteral(value) => {
                    RawPattern::primitive_literal(value, atom.range)
                }
                HuskyAtomVariant::WordPattern { .. } => todo!(),
                _ => err!(
                    format!("expect pattern but got `{:?}` instead", atom),
                    atom.range
                )?,
            })
        } else {
            None
        })
    }

    fn next_pattern_opr(&mut self) -> Option<(PatternOpr, TextRange)> {
        let atom = &self.next_atom()?;
        let opr = match atom.variant {
            HuskyAtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::BitOr)) => PatternOpr::Or,
            _ => {
                p!(atom.variant);
                todo!()
            }
        };
        Some((opr, atom.range))
    }

    fn next_atom(&mut self) -> Option<&HuskyAtom> {
        if self.next >= self.atoms.len() {
            None
        } else {
            let next = self.next;
            self.next += 1;
            Some(&self.atoms[next])
        }
    }

    fn parse_into_pattern(mut self) -> AtomResult<(RawPattern, Vec<HuskyAtom>)> {
        Ok((
            self.parse_pattern()?,
            self.atoms.drain(self.next..).collect(),
        ))
    }
}
