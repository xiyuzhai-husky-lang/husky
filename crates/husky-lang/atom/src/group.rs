mod convexity;

use crate::*;

use convexity::Convexity;
use scope::{Scope, ScopeKind};
use text::TextPosition;
use word::BuiltinIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomGroup {
    attr: GroupAttr,
    atoms: Vec<Atom>,
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct GroupAttr {
    pub keyword: Option<Keyword>,
    pub is_head: bool,
}

impl std::fmt::Debug for GroupAttr {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{keyword: {:?}, is_head: {:?}}}",
            &self.keyword, &self.is_head
        ))
    }
}

// new
impl AtomGroup {
    pub(crate) fn new(keyword: Option<Keyword>, is_head: bool) -> Self {
        Self {
            attr: GroupAttr { keyword, is_head },
            atoms: Vec::new(),
        }
    }
}

// get
impl AtomGroup {
    pub(crate) fn convexity(&self) -> Convexity {
        if let Some(atom) = self.atoms.last() {
            convexity::right_side_convexity(&atom.kind)
        } else {
            Convexity::Concave
        }
    }

    pub(crate) fn is_convex(&self) -> bool {
        self.convexity() == Convexity::Convex
    }

    pub fn attr(&self) -> GroupAttr {
        self.attr
    }
    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }
}

// push
impl AtomGroup {
    pub(crate) fn push(&mut self, atom: Atom) -> Result<(), AtomError> {
        if convexity::compatible(self.convexity(), convexity::left_side_convexity(&atom.kind)) {
            self.atoms.push(atom);
            Ok(())
        } else {
            Err(AtomError::new(
                atom.text_range(),
                AtomRule::CompatibleConvexity,
            ))
        }
    }

    pub(crate) fn end_list(&mut self, ket: Bracket, attr: ListEndAttr, text_range: TextRange) {
        if self.is_convex() {
            self.push(Atom::new(text_range.clone(), AtomKind::ListItem))
                .unwrap();
        }
        self.push(Atom::new(text_range, AtomKind::ListEnd(ket, attr)))
            .unwrap();
    }

    pub(crate) fn start_list(&mut self, bra: Bracket, text_range: TextRange) {
        self.push(Atom::new(
            text_range,
            AtomKind::ListStart(
                bra,
                if self.is_convex() {
                    ListStartAttr::Attach
                } else {
                    ListStartAttr::None
                }
                .into(),
            ),
        ))
        .unwrap();
    }

    pub(crate) fn start_lambda(&mut self, text_range: TextRange) -> Result<(), AtomError> {
        self.push(Atom::new(
            text_range,
            AtomKind::ListStart(Bracket::Vert, ListStartAttr::None),
        ))
    }

    pub(crate) fn end_lambda(&mut self, text_range: TextRange) {
        self.end_list(Bracket::Vert, ListEndAttr::Attach, text_range)
    }

    pub(crate) fn make_default_routine_type(
        &mut self,
        db: &dyn AtomQuery,
        output: ScopeId,
        ket_to_output: &TextRange,
    ) -> Result<(), AtomError> {
        let mut generic_arguments = Vec::new();
        let range = loop {
            let type_atom = self.atoms.pop().ok_or(AtomError::new(
                ket_to_output.clone(),
                AtomRule::BracketsShouldMatch,
            ))?;
            match type_atom.kind {
                AtomKind::ListStart(bra, attr) => {
                    if bra != Bracket::Par {
                        Err(AtomError::new(
                            type_atom.text_start()..(ket_to_output.end),
                            AtomRule::BracketsShouldMatch,
                        ))?
                    }
                    if attr != ListStartAttr::None {
                        Err(AtomError::new(
                            type_atom.text_start()..(ket_to_output.end),
                            AtomRule::CompatibleConvexity,
                        ))?
                    }
                    break type_atom.text_start()..(ket_to_output.end);
                }
                AtomKind::Scope(scope_id, scope_kind) => {
                    if scope_kind != ScopeKind::Type {
                        return Err(AtomError::new(
                            type_atom.text_start()..(ket_to_output.end),
                            AtomRule::OnlyTypesInTheParenthesisBeforeLightArrow,
                        ))?;
                    };
                    generic_arguments.insert(0, scope_id)
                }
                // AtomKind::ListItem => todo!(),
                _ => Err(AtomError::new(
                    type_atom.text_start()..(ket_to_output.end),
                    AtomRule::OnlyTypesInTheParenthesisBeforeLightArrow,
                ))?,
            }
            let bra_or_separator_atom = self.atoms.pop().ok_or(AtomError::new(
                ket_to_output.clone(),
                AtomRule::BracketsShouldMatch,
            ))?;
            match bra_or_separator_atom.kind {
                AtomKind::ListStart(bra, attr) => {
                    if bra != Bracket::Par {
                        Err(AtomError::new(
                            bra_or_separator_atom.text_start()..(ket_to_output.end),
                            AtomRule::BracketsShouldMatch,
                        ))?
                    }
                    if attr != ListStartAttr::None {
                        Err(AtomError::new(
                            bra_or_separator_atom.text_start()..(ket_to_output.end),
                            AtomRule::CompatibleConvexity,
                        ))?
                    }
                    break bra_or_separator_atom.text_start()..(ket_to_output.end);
                }
                AtomKind::ListItem => todo!(),
                _ => Err(AtomError::new(
                    bra_or_separator_atom.text_start()..(ket_to_output.end),
                    AtomRule::ListShouldBeWellFormed,
                ))?,
            }
        };

        generic_arguments.push(output);
        let routine_type = db.scope_to_id(Scope::builtin(
            word::default_routine_type(),
            Some(generic_arguments),
        ));
        self.push(Atom::new(
            range,
            AtomKind::Scope(routine_type, ScopeKind::Type),
        ));
        Ok(())
    }
}
