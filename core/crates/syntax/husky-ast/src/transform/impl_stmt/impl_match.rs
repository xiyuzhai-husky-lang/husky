use wild_utils::ref_to_mut_ref;

use super::*;
use crate::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_match(&mut self, token_group: &[HuskyToken]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        let match_expr = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        Ok(RawStmtVariant::Match {
            match_expr,
            match_liason: MatchLiason::Pure,
        })
    }

    pub(super) fn parse_case(&mut self, token_group: &[HuskyToken]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        if token_group.len() < 3 {
            return err!("expect `case <pattern>:`", token_group.text_range());
        }
        let atoms = self.parse_atoms(&token_group[1..(token_group.len() - 1)], |parser| {
            parser.parse_all()
        })?;
        Ok(RawStmtVariant::PatternBranch {
            pattern_branch_variant: RawPatternBranchVariant::Case {
                pattern: MatchPatternParser::new(&atoms).parse()?,
            },
        })
    }
}

pub struct MatchPatternParser<'a> {
    atom_iter: std::slice::Iter<'a, HuskyAtom>,
}

impl<'a> MatchPatternParser<'a> {
    pub fn new(atoms: &'a [HuskyAtom]) -> Self {
        Self {
            atom_iter: atoms.iter(),
        }
    }

    pub fn parse(mut self) -> AstResult<RawCasePattern> {
        let mut pattern = self.parse_simple_pattern()?.unwrap();
        while let Some(separator) = self.atom_iter.next() {
            match separator.variant {
                AtomVariant::Binary(BinaryOpr::Pure(PureBinaryOpr::BitOr)) => {
                    if let Some(new_pattern) = self.parse_simple_pattern()? {
                        pattern = pattern.or(new_pattern)?
                    } else {
                        return err!("expect pattern after `|`", separator.range);
                    }
                }
                _ => todo!(),
            }
        }
        Ok(pattern)
    }

    fn parse_simple_pattern(&mut self) -> AstResult<Option<RawCasePattern>> {
        Ok(if let Some(atom) = self.atom_iter.next() {
            Some(match atom.variant {
                AtomVariant::EntityRoute { route, kind } => match kind {
                    EntityKind::EnumLiteral => RawCasePattern::enum_literal(route, atom.range),
                    _ => err!(format!("expect enum literal"), atom.range)?,
                },
                AtomVariant::PrimitiveLiteral(value) => {
                    RawCasePattern::primitive_literal(value, atom.range)
                }
                _ => err!(format!("expect pattern"), atom.range)?,
            })
        } else {
            None
        })
    }
}
