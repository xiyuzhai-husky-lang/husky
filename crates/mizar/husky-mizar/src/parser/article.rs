use super::*;

pub(super) struct ArticleParser<'a> {
  pub r: MizReader<'a>,
  pub buf: Vec<u8>,
}

#[derive(Debug)]
enum ArticleElem {
  Item(Item),
  AuxiliaryItem(AuxiliaryItem),
  Let(Vec<Type>),
  Assume(Vec<Proposition>),
  Given(GivenItem),
  Take(Term),
  TakeAsVar(Type, Term),
  Definition(Definition),
  DefStruct(DefStruct),
  Canceled(CancelKind),
  Thus(Statement),
  PerCases(PerCases),
  PerCasesJustification(Proposition, Justification),
  Registration(Registration),
  SimpleCorrCond(SimpleCorrCond),
  CorrCond(CorrCond),
  Correctness(Correctness),
  JustifiedProperty(JustifiedProperty),
  Constructor(ConstructorDef),
  Pattern(Pattern),
  BlockThesis(Formula),
  Proposition(Proposition),
  CaseBlock(CaseBlock),
  Case(CaseKind),
  EndPosition(Position),
  Other,
  End,
}

impl From<ArticleElem> for Item {
  fn from(value: ArticleElem) -> Self {
    match value {
      ArticleElem::Item(it) => it,
      ArticleElem::AuxiliaryItem(it) => Item::AuxiliaryItem(it),
      ArticleElem::Let(it) => Item::Let(it),
      ArticleElem::Assume(it) => Item::Assume(it),
      ArticleElem::Given(it) => Item::Given(it),
      ArticleElem::Take(tm) => Item::Take(tm),
      ArticleElem::TakeAsVar(ty, tm) => Item::TakeAsVar(ty, tm),
      ArticleElem::Definition(decl) => Item::Definition(decl),
      ArticleElem::DefStruct(decl) => Item::DefStruct(decl),
      ArticleElem::Canceled(it) => Item::Canceled(it),
      ArticleElem::Thus(s) => Item::Thus(s),
      ArticleElem::PerCases(it) => Item::PerCases(it),
      ArticleElem::Registration(it) => Item::Registration(it),
      ArticleElem::Pattern(it) => Item::Pattern(it),
      ArticleElem::CorrCond(_)
      | ArticleElem::SimpleCorrCond(_)
      | ArticleElem::Correctness(_)
      | ArticleElem::JustifiedProperty(_)
      | ArticleElem::Constructor(_)
      | ArticleElem::BlockThesis(_)
      | ArticleElem::Proposition(_)
      | ArticleElem::PerCasesJustification(..)
      | ArticleElem::CaseBlock(_)
      | ArticleElem::Case(_)
      | ArticleElem::EndPosition(_)
      | ArticleElem::Other
      | ArticleElem::End => unreachable!(),
    }
  }
}

impl ArticleParser<'_> {
  pub fn parse_item(&mut self) -> Result<Option<Item>> {
    let idx = self.r.position();
    Ok(match self.parse_elem()? {
      e @ (ArticleElem::Item(_) | ArticleElem::AuxiliaryItem(_) | ArticleElem::Canceled(_)) =>
        Some(e.into()),
      ArticleElem::Proposition(prop) => Some(self.finish_proposition(prop)?),
      ArticleElem::End => {
        assert!(matches!(self.r.read_event(&mut self.buf)?, Event::Eof));
        None
      }
      _ => return Err(ParseError::unexpected_elem(idx, "item", None)),
    })
  }

  fn finish_proposition(&mut self, prop: Proposition) -> Result<Item> {
    let s = Statement::Proposition { prop, just: self.parse_justification()? };
    Ok(Item::AuxiliaryItem(AuxiliaryItem::Statement(s)))
  }

  fn parse_block_thesis(&mut self) -> Result<Option<Formula>> {
    Ok(match self.parse_elem()? {
      ArticleElem::BlockThesis(f) => Some(f),
      _ => None,
    })
  }

  fn parse_thesis(&mut self) -> Result<Thesis> {
    let idx = self.r.position();
    let Elem::Thesis(f) = self.r.parse_elem(&mut self.buf)? else {
      return Err(ParseError::unexpected_elem(idx, "thesis", None))
    };
    Ok(f)
  }

  #[allow(clippy::type_complexity)]
  fn parse_reasoning(&mut self, diffuse: bool) -> Result<(Vec<(Item, Option<Thesis>)>, Position)> {
    let mut items = vec![];
    let end = loop {
      let e = self.parse_elem()?;
      match e {
        ArticleElem::AuxiliaryItem(_) | ArticleElem::PerCases(_) => items.push((e.into(), None)),
        ArticleElem::Proposition(prop) => items.push((self.finish_proposition(prop)?, None)),
        ArticleElem::Let(_)
        | ArticleElem::Thus(_)
        | ArticleElem::Assume(_)
        | ArticleElem::Given(_)
        | ArticleElem::Take(_)
        | ArticleElem::TakeAsVar(..) => {
          let mut thesis = None;
          if !diffuse {
            thesis = Some(self.parse_thesis()?)
          }
          items.push((e.into(), thesis));
        }
        ArticleElem::EndPosition(pos) => break pos,
        _ => panic!("unexpected reasoning element"),
      }
    };
    Ok((items, end))
  }

  fn parse_references(&mut self) -> Result<Vec<Reference>> {
    let mut refs = vec![];
    while let Ok(e) = self.r.try_read_start(&mut self.buf, Some("Ref"))? {
      let (pos, label) = self.r.get_pos_and_label(&e)?;
      let (mut article_nr, mut nr, mut kind) = Default::default();
      for attr in e.attributes() {
        let attr = attr?;
        match attr.key.0 {
          b"kind" => kind = attr.value[0],
          b"nr" => nr = self.r.get_attr::<u32>(&attr.value)? - 1,
          b"articlenr" => article_nr = self.r.get_attr(&attr.value)?,
          _ => {}
        }
      }
      #[allow(clippy::panic)]
      #[allow(clippy::unwrap_used)]
      let kind = match kind {
        0 => ReferenceKind::Priv(label.expect("private reference missing label")),
        b'T' => ReferenceKind::Thm((ArticleId(article_nr), ThmId(nr))),
        b'D' => ReferenceKind::Def((ArticleId(article_nr), DefId(nr))),
        _ => panic!("unexpected inference kind"),
      };
      self.r.end_tag(&mut self.buf)?;
      refs.push(Reference { pos, kind });
    }
    Ok(refs)
  }

  fn parse_justification(&mut self) -> Result<Justification> {
    let e = self.r.read_start(&mut self.buf, None)?;
    Ok(match e.local_name().as_ref() {
      b"By" => {
        let linked =
          e.try_get_attribute(b"linked").unwrap().map_or(false, |attr| &*attr.value == b"true");
        Justification::Simple(Inference {
          kind: InferenceKind::By { linked },
          pos: self.r.get_pos(&e)?,
          refs: self.parse_references()?,
        })
      }
      b"From" => {
        let (mut article_nr, mut nr) = Default::default();
        for attr in e.attributes() {
          let attr = attr?;
          match attr.key.0 {
            b"nr" => nr = self.r.get_attr::<u32>(&attr.value)? - 1,
            b"articlenr" => article_nr = self.r.get_attr(&attr.value)?,
            _ => {}
          }
        }
        Justification::Simple(Inference {
          kind: InferenceKind::From { sch: (ArticleId(article_nr), SchId(nr)) },
          pos: self.r.get_pos(&e)?,
          refs: self.parse_references()?,
        })
      }
      b"Proof" => {
        let (start, label) = self.r.get_pos_and_label(&e)?;
        let thesis = self.parse_block_thesis()?.unwrap();
        let (items, end) = self.parse_reasoning(false)?;
        self.r.end_tag(&mut self.buf)?;
        Justification::Proof { pos: (start, end), label, thesis, items }
      }
      b"SkippedProof" => {
        self.r.end_tag(&mut self.buf)?;
        Justification::SkippedProof
      }
      _ => panic!("unexpected justification"),
    })
  }

  fn parse_simple_justification(&mut self) -> Result<Inference> {
    Ok(match self.parse_justification()? {
      Justification::Simple(just) => just,
      _ => panic!("expected simple justification"),
    })
  }

  fn parse_end_pos(&mut self) -> Result<Position> {
    let ArticleElem::EndPosition(pos) = self.parse_elem()? else {
      panic!("expected <EndPosition>")
    };
    Ok(pos)
  }

  fn parse_corr_cond(&mut self, kind: CorrCondKind) -> Result<ArticleElem> {
    Ok(match self.r.parse_elem(&mut self.buf)? {
      Elem::Formula(f) => {
        self.r.end_tag(&mut self.buf)?;
        ArticleElem::SimpleCorrCond(SimpleCorrCond { kind, f })
      }
      Elem::Proposition(prop) => {
        let just = self.parse_justification()?;
        self.r.end_tag(&mut self.buf)?;
        ArticleElem::CorrCond(CorrCond { kind, prop, just })
      }
      _ => panic!("invalid correctness condition"),
    })
  }

  fn parse_cond_and_correctness(&mut self) -> Result<(Vec<CorrCond>, Option<Correctness>)> {
    let mut conds = vec![];
    Ok(loop {
      match self.parse_elem()? {
        ArticleElem::CorrCond(cond) => conds.push(cond),
        ArticleElem::Correctness(corr) => {
          self.r.end_tag(&mut self.buf)?;
          break (conds, Some(corr))
        }
        ArticleElem::End => break (conds, None),
        _ => panic!("expected <Correctness>"),
      }
    })
  }

  fn parse_case_block(&mut self, (start, label): (Position, Option<LabelId>)) -> Result<CaseBlock> {
    assert!(label.is_none());
    let mut block_thesis = None;
    let cs = loop {
      match self.parse_elem()? {
        ArticleElem::BlockThesis(f) => assert!(block_thesis.replace(f).is_none()),
        ArticleElem::Case(cs) => break cs,
        _ => panic!("expected <Case> / <Suppose>"),
      }
    };
    let thesis = match block_thesis {
      Some(_) => Some(self.parse_thesis()?),
      None => None,
    };
    let (items, end) = self.parse_reasoning(block_thesis.is_none())?;
    let block_thesis = match block_thesis {
      Some(block_thesis) => block_thesis,
      None => self.parse_block_thesis()?.unwrap(),
    };
    self.r.end_tag(&mut self.buf)?;
    Ok(CaseBlock { pos: (start, end), block_thesis, cs, items, thesis })
  }

  fn parse_elem(&mut self) -> Result<ArticleElem> {
    Ok(if let Event::Start(e) = self.r.read_event(&mut self.buf)? {
      match e.local_name().as_ref() {
        b"DefinitionBlock" => {
          let (start, label) = self.r.get_pos_and_label(&e)?;
          let mut items = vec![];
          let end = loop {
            let e = self.parse_elem()?;
            match e {
              ArticleElem::AuxiliaryItem(_)
              | ArticleElem::Let(_)
              | ArticleElem::Assume(_)
              | ArticleElem::Given(_)
              | ArticleElem::Definition(_)
              | ArticleElem::DefStruct(_)
              | ArticleElem::Canceled(_) => items.push(e.into()),
              ArticleElem::Proposition(prop) => items.push(self.finish_proposition(prop)?),
              ArticleElem::EndPosition(pos) => break pos,
              e => panic!("unexpected definition item {e:?}"),
            }
          };
          self.r.end_tag(&mut self.buf)?;
          let kind = BlockKind::Definition;
          ArticleElem::Item(Item::Block { kind, pos: (start, end), label, items })
        }
        b"RegistrationBlock" => {
          let (start, label) = self.r.get_pos_and_label(&e)?;
          let mut items = vec![];
          let end = loop {
            let e = self.parse_elem()?;
            match e {
              ArticleElem::AuxiliaryItem(_)
              | ArticleElem::Let(_)
              | ArticleElem::Registration(_)
              | ArticleElem::Canceled(_) => items.push(e.into()),
              ArticleElem::Proposition(prop) => items.push(self.finish_proposition(prop)?),
              ArticleElem::EndPosition(pos) => break pos,
              _ => panic!("unexpected definition item"),
            }
          };
          self.r.end_tag(&mut self.buf)?;
          let kind = BlockKind::Registration;
          ArticleElem::Item(Item::Block { kind, pos: (start, end), label, items })
        }
        b"NotationBlock" => {
          let (start, label) = self.r.get_pos_and_label(&e)?;
          let mut items = vec![];
          let end = loop {
            let e = self.parse_elem()?;
            match e {
              ArticleElem::AuxiliaryItem(_) | ArticleElem::Let(_) | ArticleElem::Pattern(_) =>
                items.push(e.into()),
              ArticleElem::Proposition(prop) => items.push(self.finish_proposition(prop)?),
              ArticleElem::EndPosition(pos) => break pos,
              _ => panic!("unexpected definition item"),
            }
          };
          self.r.end_tag(&mut self.buf)?;
          let kind = BlockKind::Notation;
          ArticleElem::Item(Item::Block { kind, pos: (start, end), label, items })
        }
        b"Reservation" => {
          // Note: <Var> elements inside reservations are seriously broken. For example, in:
          //   reserve A for set, f for Function of A, {x where x is Nat : x = A};
          // both x and A in the equality are represented as <Var nr="1"/>.
          // Luckily, the checker doesn't really care about reservations, so we just skip them.
          self.r.suppress_bvar_errors = true;
          let mut ids = vec![];
          let ty = loop {
            match self.r.parse_elem(&mut self.buf)? {
              Elem::Ident(id) => ids.push(id),
              Elem::Type(ty) => break Box::new(ty),
              _ => panic!("unexpected reservation item"),
            }
          };
          self.r.suppress_bvar_errors = false;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Item(Item::Reservation { ids, ty })
        }
        b"SchemeBlock" => {
          let start = self.r.get_pos(&e)?;
          let nr =
            self.r.get_attr::<u32>(&e.try_get_attribute(b"schemenr").unwrap().unwrap().value)? - 1;
          let mut decls = vec![];
          loop {
            if let Event::Start(e) = self.r.read_event(&mut self.buf)? {
              match e.local_name().as_ref() {
                b"SchemeFuncDecl" => {
                  let args = self.r.parse_arg_types(&mut self.buf)?;
                  let ty = self.r.parse_type(&mut self.buf)?.unwrap();
                  self.r.end_tag(&mut self.buf)?;
                  decls.push(SchemeDecl::Func { args, ty });
                }
                b"SchemePredDecl" => {
                  let args = self.r.parse_arg_types(&mut self.buf)?;
                  self.r.end_tag(&mut self.buf)?;
                  decls.push(SchemeDecl::Pred { args });
                }
                b"SchemePremises" => break,
                _ => panic!("unexpected scheme decl"),
              }
            }
          }
          let mut prems = vec![];
          while let Some(prop) = self.r.parse_proposition(&mut self.buf, true)? {
            prems.push(prop);
          }
          let thesis = self.r.parse_proposition(&mut self.buf, false)?.unwrap();
          let just = self.parse_justification()?;
          let end = self.parse_end_pos()?;
          self.r.end_tag(&mut self.buf)?;
          let bl = SchemeBlock { pos: (start, end), nr: SchId(nr), decls, prems, thesis, just };
          ArticleElem::Item(Item::Scheme(bl))
        }
        b"CaseBlock" => {
          let attrs = self.r.get_pos_and_label(&e)?;
          let bl = self.parse_case_block(attrs)?;
          assert!(matches!(bl.cs, CaseKind::Case(_)));
          ArticleElem::CaseBlock(bl)
        }
        b"SupposeBlock" => {
          let attrs = self.r.get_pos_and_label(&e)?;
          let bl = self.parse_case_block(attrs)?;
          assert!(matches!(bl.cs, CaseKind::Suppose(_)));
          ArticleElem::CaseBlock(bl)
        }
        b"Case" => {
          let mut props = vec![];
          while let Some(prop) = self.r.parse_proposition(&mut self.buf, true)? {
            props.push(prop)
          }
          ArticleElem::Case(CaseKind::Case(props))
        }
        b"Suppose" => {
          let mut props = vec![];
          while let Some(prop) = self.r.parse_proposition(&mut self.buf, true)? {
            props.push(prop)
          }
          ArticleElem::Case(CaseKind::Suppose(props))
        }
        b"JustifiedTheorem" => {
          let prop = self.r.parse_proposition(&mut self.buf, true)?.unwrap();
          let just = self.parse_justification()?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Item(Item::Theorem { prop, just })
        }
        b"DefTheorem" => {
          let kind = self.r.parse_constr_kind(&e)?;
          let prop = self.r.parse_proposition(&mut self.buf, true)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Item(Item::DefTheorem { kind, prop })
        }
        b"Definiens" => {
          let attrs = self.r.parse_definiens_attrs(e)?;
          let df = self.r.parse_definiens_body(&mut self.buf, attrs)?;
          ArticleElem::Item(Item::Definiens(df))
        }
        b"Proposition" => {
          let (pos, label) = self.r.get_pos_and_label(&e)?;
          let f = self.r.parse_formula(&mut self.buf)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Proposition(Proposition { pos, label, f })
        }
        b"IterEquality" => {
          let (start, label) = self.r.get_pos_and_label(&e)?;
          let lhs = self.r.parse_term(&mut self.buf)?.unwrap();
          let mut steps = vec![];
          while self.r.try_read_start(&mut self.buf, Some("IterStep"))?.is_ok() {
            let rhs = self.r.parse_term(&mut self.buf)?.unwrap();
            let just = self.parse_simple_justification()?;
            self.r.end_tag(&mut self.buf)?;
            steps.push((rhs, just));
          }
          let s = Statement::IterEquality { start, label, lhs, steps };
          ArticleElem::AuxiliaryItem(AuxiliaryItem::Statement(s))
        }
        b"Now" => {
          let (start, label) = self.r.get_pos_and_label(&e)?;
          let (items, end) = self.parse_reasoning(true)?;
          let thesis = self.parse_block_thesis()?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          let s = Statement::Now {
            pos: (start, end),
            label,
            thesis,
            items: items.into_iter().map(|p| p.0).collect(),
          };
          ArticleElem::AuxiliaryItem(AuxiliaryItem::Statement(s))
        }
        b"Consider" => {
          let prop = self.r.parse_proposition(&mut self.buf, false)?.unwrap();
          let just = self.parse_justification()?;
          let (mut fixed, mut parsing_fixed, mut intro) = (vec![], true, vec![]);
          loop {
            match self.r.parse_elem(&mut self.buf)? {
              Elem::Type(ty) if parsing_fixed => fixed.push(ty),
              Elem::Proposition(prop) => {
                parsing_fixed = false;
                intro.push(prop)
              }
              Elem::End => break,
              _ => panic!("expected proposition"),
            }
          }
          ArticleElem::AuxiliaryItem(AuxiliaryItem::Consider { prop, just, fixed, intro })
        }
        b"Set" => {
          let term = self.r.parse_term(&mut self.buf)?.unwrap();
          let ty = self.r.parse_type(&mut self.buf)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::AuxiliaryItem(AuxiliaryItem::Set { term, ty })
        }
        b"Reconsider" => {
          let mut terms = vec![];
          let prop = loop {
            match self.r.parse_elem(&mut self.buf)? {
              Elem::Type(ty) => terms.push((ty, self.r.parse_term(&mut self.buf)?.unwrap())),
              Elem::Proposition(prop) => break prop,
              _ => panic!("expected proposition"),
            }
          };
          assert!(prop.label.is_none());
          let just = self.parse_justification()?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::AuxiliaryItem(AuxiliaryItem::Reconsider { terms, prop, just })
        }
        b"DefFunc" => {
          let args = self.r.parse_arg_types(&mut self.buf)?;
          let value = self.r.parse_term(&mut self.buf)?.unwrap();
          let ty = self.r.parse_type(&mut self.buf)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::AuxiliaryItem(AuxiliaryItem::DefFunc { args, value, ty })
        }
        b"DefPred" => {
          let args = self.r.parse_arg_types(&mut self.buf)?;
          let value = self.r.parse_formula(&mut self.buf)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::AuxiliaryItem(AuxiliaryItem::DefPred { args, value })
        }
        b"Let" => {
          let mut fixed = vec![];
          while let Some(ty) = self.r.parse_type(&mut self.buf)? {
            fixed.push(ty)
          }
          ArticleElem::Let(fixed)
        }
        b"Assume" => {
          let mut props = vec![];
          while let Some(prop) = self.r.parse_proposition(&mut self.buf, true)? {
            props.push(prop)
          }
          ArticleElem::Assume(props)
        }
        b"Given" => {
          let prop = self.r.parse_proposition(&mut self.buf, false)?.unwrap();
          let (mut fixed, mut parsing_fixed, mut intro) = (vec![], true, vec![]);
          loop {
            match self.r.parse_elem(&mut self.buf)? {
              Elem::Type(ty) if parsing_fixed => fixed.push(ty),
              Elem::Proposition(prop) => {
                parsing_fixed = false;
                intro.push(prop)
              }
              Elem::End => break,
              _ => panic!("expected proposition"),
            }
          }
          ArticleElem::Given(GivenItem { prop, fixed, intro })
        }
        b"Definition" => {
          let (pos, label) = self.r.get_pos_and_label(&e)?;
          let (mut expand, mut redef, mut kind) = Default::default();
          for attr in e.attributes() {
            let attr = attr?;
            match attr.key.0 {
              b"kind" => kind = attr.value[0],
              b"expandable" => expand = &*attr.value == b"true",
              b"redefinition" => redef = &*attr.value == b"true",
              _ => {}
            }
          }
          if kind == b'G' {
            assert!(!expand && !redef);
            let mut constrs = vec![];
            let cl = loop {
              match self.parse_elem()? {
                ArticleElem::Constructor(it) => constrs.push(it),
                ArticleElem::Registration(Registration::Cluster(decl)) => break decl,
                _ => panic!("expected cluster"),
              }
            };
            let (mut conds, mut corr, mut patts, mut state) = (vec![], None, vec![], false);
            loop {
              match self.parse_elem()? {
                ArticleElem::CorrCond(it) if !state => conds.push(it),
                ArticleElem::Correctness(it) if !state => {
                  state = true;
                  corr = Some(it)
                }
                ArticleElem::Pattern(it) => {
                  state = true;
                  patts.push(it)
                }
                ArticleElem::End => break,
                _ => panic!("expected correctness condition or pattern"),
              }
            }
            ArticleElem::DefStruct(DefStruct { pos, label, constrs, cl, conds, corr, patts })
          } else {
            let kind = match (expand, kind) {
              (false, b'V') => DefinitionKind::PrAttr,
              (false, b'M') => DefinitionKind::Mode,
              (false, b'R') => DefinitionKind::Pred,
              (false, b'K') => DefinitionKind::Func,
              (true, b'M') => DefinitionKind::ExpandMode,
              _ => panic!("unexpected definition kind"),
            };
            let (mut conds, mut corr, mut props, mut constr, mut patts, mut state) =
              (vec![], None, vec![], None, vec![], 0u8);
            loop {
              match self.parse_elem()? {
                ArticleElem::CorrCond(it) if state == 0 => conds.push(it),
                ArticleElem::Correctness(it) if state == 0 => {
                  state = 1;
                  corr = Some(it)
                }
                ArticleElem::JustifiedProperty(it) if state <= 1 => {
                  state = 1;
                  props.push(it)
                }
                ArticleElem::Constructor(it) if state <= 1 => {
                  state = 2;
                  constr = Some(it)
                }
                ArticleElem::Pattern(it) if state <= 2 => {
                  state = 3;
                  patts.push(it)
                }
                ArticleElem::End => break,
                _ => panic!("expected correctness condition or pattern"),
              }
            }
            let d = Definition { pos, label, redef, kind, props, conds, corr, constr, patts };
            ArticleElem::Definition(d)
          }
        }
        b"Canceled" => {
          let kind = e.try_get_attribute(b"kind").unwrap().unwrap().value[0];
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Canceled(match kind {
            b'D' => CancelKind::Def,
            b'T' => CancelKind::Thm,
            b'S' => CancelKind::Sch,
            _ => panic!("unknown cancel kind"),
          })
        }
        b"Conclusion" => {
          let s = match self.parse_elem()? {
            ArticleElem::AuxiliaryItem(AuxiliaryItem::Statement(s)) => s,
            ArticleElem::Proposition(prop) =>
              Statement::Proposition { prop, just: self.parse_justification()? },
            _ => panic!("expected justified proposition"),
          };
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Thus(s)
        }
        b"Take" => {
          let tm = self.r.parse_term(&mut self.buf)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Take(tm)
        }
        b"TakeAsVar" => {
          let ty = self.r.parse_type(&mut self.buf)?.unwrap();
          let tm = self.r.parse_term(&mut self.buf)?.unwrap();
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::TakeAsVar(ty, tm)
        }
        b"PerCasesReasoning" => {
          let (start, label) = self.r.get_pos_and_label(&e)?;
          let mut block_thesis = None;
          let mut cases = vec![];
          let (prop, just) = loop {
            match self.parse_elem()? {
              ArticleElem::BlockThesis(f) => assert!(block_thesis.replace(f).is_none()),
              ArticleElem::CaseBlock(cs) => cases.push(cs),
              ArticleElem::PerCasesJustification(prop, just) => break (prop, just),
              _ => panic!("expected <PerCases>"),
            }
          };
          let thesis = match block_thesis {
            None => None,
            Some(_) => Some(self.parse_thesis()?),
          };
          let pos = (start, self.parse_end_pos()?);
          let block_thesis = match block_thesis {
            Some(block_thesis) => block_thesis,
            None => self.parse_block_thesis()?.unwrap(),
          };
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::PerCases(PerCases { label, cases, prop, just, thesis, pos, block_thesis })
        }
        b"PerCases" => {
          let prop = self.r.parse_proposition(&mut self.buf, false)?.unwrap();
          let just = self.parse_justification()?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::PerCasesJustification(prop, just)
        }
        b"Registration" => {
          let e = self.r.read_start(&mut self.buf, None)?;
          let kind = match self.r.parse_cluster_attrs(&e)? {
            (aid, ClusterKind::R) => ClusterDeclKind::R(self.r.parse_rcluster(&mut self.buf, aid)?),
            (aid, ClusterKind::F) => ClusterDeclKind::F(self.r.parse_fcluster(&mut self.buf, aid)?),
            (aid, ClusterKind::C) => ClusterDeclKind::C(self.r.parse_ccluster(&mut self.buf, aid)?),
          };
          let (conds, corr) = self.parse_cond_and_correctness()?;
          ArticleElem::Registration(Registration::Cluster(ClusterDecl { kind, conds, corr }))
        }
        b"IdentifyRegistration" => {
          let e = self.r.read_start(&mut self.buf, Some("Identify"))?;
          let attrs = self.r.parse_identify_attrs(&e)?;
          let kind = self.r.parse_identify_body(&mut self.buf, attrs)?;
          let (conds, corr) = self.parse_cond_and_correctness()?;
          ArticleElem::Registration(Registration::Identify { kind, conds, corr })
        }
        b"ReductionRegistration" => {
          let e = self.r.read_start(&mut self.buf, Some("Reduction"))?;
          let attrs = self.r.parse_reduction_attrs(&e)?;
          let kind = self.r.parse_reduction_body(&mut self.buf, attrs)?;
          let (conds, corr) = self.parse_cond_and_correctness()?;
          ArticleElem::Registration(Registration::Reduction { kind, conds, corr })
        }
        b"PropertyRegistration" => {
          let e = self.r.read_start(&mut self.buf, Some("Property"))?;
          let attrs = self.r.parse_property_attrs(&e)?;
          let kind = self.r.parse_property_body(&mut self.buf, attrs)?;
          let prop = self.r.parse_proposition(&mut self.buf, false)?.unwrap();
          let just = self.parse_justification()?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Registration(Registration::Property { kind, prop, just })
        }
        b"Coherence" => return self.parse_corr_cond(CorrCondKind::Coherence),
        b"Compatibility" => return self.parse_corr_cond(CorrCondKind::Compatibility),
        b"Consistency" => return self.parse_corr_cond(CorrCondKind::Consistency),
        b"Existence" => return self.parse_corr_cond(CorrCondKind::Existence),
        b"Uniqueness" => return self.parse_corr_cond(CorrCondKind::Uniqueness),
        b"Reducibility" => return self.parse_corr_cond(CorrCondKind::Reducibility),
        b"Correctness" => {
          let mut conds = vec![];
          let prop = loop {
            match self.parse_elem()? {
              ArticleElem::SimpleCorrCond(it) => conds.push(it),
              ArticleElem::Proposition(prop) => break prop,
              _ => panic!("expected proposition"),
            }
          };
          let just = self.parse_justification()?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::Correctness(Correctness { conds, prop, just })
        }
        b"JustifiedProperty" => {
          let e = self.r.read_start(&mut self.buf, None)?;
          let kind = e.local_name().as_ref().try_into().expect("unexpected property");
          self.r.end_tag(&mut self.buf)?;
          let prop = self.r.parse_proposition(&mut self.buf, false)?.unwrap();
          let just = self.parse_justification()?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::JustifiedProperty(JustifiedProperty { kind, prop, just })
        }
        b"Constructor" => {
          let attrs = self.r.parse_constructor_attrs(&e)?;
          ArticleElem::Constructor(self.r.parse_constructor_body(&mut self.buf, attrs)?)
        }
        b"Pattern" => {
          let attrs = self.r.parse_pattern_attrs(&e)?;
          ArticleElem::Pattern(self.r.parse_pattern_body(&mut self.buf, attrs, |x| x)?)
        }
        b"BlockThesis" => {
          // Note: It seems to be somewhat rare, but there is a bvar error in the block thesis
          // at ring_2.miz:2267 indicating some kind of bvar bug in the analyzer.
          // This is in the <Thesis> element which is ignored here, so we just suppress the error.
          self.r.suppress_bvar_errors = true;
          let f = loop {
            match self.r.parse_elem(&mut self.buf)? {
              Elem::Formula(f) => break f,
              Elem::Thesis(_) => {}
              _ => panic!("unexpected block thesis element"),
            }
          };
          self.r.suppress_bvar_errors = false;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::BlockThesis(f)
        }
        b"EndPosition" => {
          let end = self.r.get_pos(&e)?;
          self.r.end_tag(&mut self.buf)?;
          ArticleElem::EndPosition(end)
        }
        _ => ArticleElem::Other,
      }
    } else {
      ArticleElem::End
    })
  }
}
