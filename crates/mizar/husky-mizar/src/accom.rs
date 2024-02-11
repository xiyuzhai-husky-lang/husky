use crate::error::report_accom_warning;
use crate::parser::{catch_missing, ParseError, PathResult};
use crate::reader::DefiniensId;
use crate::types::*;
use crate::{mk_id, CmpStyle, MizPath, VisitMut, MML_VCT_PATH};
use std::collections::HashMap;
use std::io;

mk_id! {
  MizVocId(u32),
  SigId(u32),
}

#[derive(Debug, Default)]
struct MizVocBuilder {
    pub voc: MizIdxVec<MizVocId, (Article, SymbolsBase)>,
    base: SymbolsBase,
}

impl MizVocBuilder {
    fn push(&mut self, art: Article, val: &SymbolsBase) -> MizVocId {
        let i = self.voc.push((art, self.base));
        self.base += val;
        i
    }

    fn get_or_push(&mut self, art: Article, val: &SymbolsBase) -> MizVocId {
        let res = self.voc.enum_iter().find(|p| p.1 .0 == art);
        if let Some((i, (_, val2))) = res {
            assert_eq!(*self.hi(i) - val2, *val);
            i
        } else {
            self.push(art, val)
        }
    }

    fn hi(&self, id: MizVocId) -> &SymbolsBase {
        self.voc
            .get(MizVocId(id.0 + 1))
            .map_or(&self.base, |(_, base)| base)
    }

    fn truncate(&mut self, len: usize) {
        if let Some(voc) = self.voc.0.get_mut(len) {
            self.base = voc.1;
            self.voc.0.truncate(len)
        }
    }
}

#[derive(Debug, Default)]
pub struct SigBuilder {
    pub sig: MizIdxVec<SigId, (Article, ConstructorsBase)>,
    pub base: ConstructorsBase,
}

impl SigBuilder {
    fn push(&mut self, constrs: Option<&mut Constructors>, art: Article) -> PathResult<SigId> {
        let mut dco = Default::default();
        MizPath { art }.read_dco(false, &mut dco, constrs.is_some())?;
        Ok(self.push_from(constrs, art, &mut dco))
    }

    fn push_from(
        &mut self,
        constrs: Option<&mut Constructors>,
        art: Article,
        dco: &mut DepConstructors,
    ) -> SigId {
        if let Some(constrs) = constrs {
            let mut rename = RenameConstr::default();
            for &art2 in &dco.sig {
                let i = self.get(art2);
                rename.push(self, i, true);
            }
            let i = self.sig.push((art, self.base));
            self.base += dco.counts;
            rename.push(self, i, true);
            dco.constrs.visit(&mut rename);
            constrs.append(&mut dco.constrs);
            i
        } else {
            let i = self.sig.push((art, self.base));
            self.base += dco.counts;
            i
        }
    }

    fn get_or_push(
        &mut self,
        constrs: Option<&mut Constructors>,
        art: Article,
    ) -> PathResult<SigId> {
        if let Some(p) = self.sig.enum_iter().find(|p| p.1 .0 == art) {
            return Ok(p.0);
        }
        self.push(constrs, art)
    }

    #[allow(clippy::panic)]
    fn get(&self, art: Article) -> SigId {
        match self.sig.enum_iter().find(|p| p.1 .0 == art) {
            Some((id, _)) => id,
            None => panic!("{art} declared out of order"),
        }
    }

    pub fn hi(&self, id: SigId) -> &ConstructorsBase {
        self.sig
            .get(SigId(id.0 + 1))
            .map_or(&self.base, |(_, base)| base)
    }

    fn rename<'a>(
        &mut self,
        sig: &[Article],
        ctx: Option<&'a Constructors>,
    ) -> PathResult<RenameConstr<'a>> {
        let mut rename = RenameConstr {
            ctx,
            ..Default::default()
        };
        let limit = self.sig.len();
        for &art in sig {
            let id = self.get_or_push(None, art)?;
            rename.push(self, id, id.into_usize() < limit)
        }
        Ok(rename)
    }

    fn truncate(&mut self, len: usize) {
        if let Some(sig) = self.sig.0.get_mut(len) {
            self.base = sig.1;
            self.sig.0.truncate(len)
        }
    }
}

#[derive(Debug, Default)]
pub struct Accomodator {
    article: Article,
    pub dirs: Directives,
    pub sig: SigBuilder,
    pub articles: HashMap<Article, ArticleId>,
    dict: MizVocBuilder,
    pub has_errors: bool,
}

#[derive(Debug, Default)]
struct RenameSymbol {
    trans: Vec<(SymbolsBase, SymbolsBase, bool)>,
    base: SymbolsBase,
    failed: bool,
}
impl RenameSymbol {
    fn push(&mut self, val: &SymbolsBase, tgt: &SymbolsBase, allow: bool) {
        self.trans.push((self.base, *tgt, allow));
        self.base += val;
    }
    fn apply(&mut self, k: SymbolKindClass, n: &mut u32) {
        #[allow(clippy::indexing_slicing)]
        if let Some(i) = self
            .trans
            .partition_point(|(base, _, _)| base.0[k] <= *n)
            .checked_sub(1)
        {
            let (from, to, allow) = &self.trans[i];
            *n = *n - from.0[k] + to.0[k];
            self.failed |= !*allow;
        }
    }
    fn ok(&mut self) -> bool {
        !std::mem::take(&mut self.failed)
    }
}

#[derive(Debug, Default)]
struct RenameConstr<'a> {
    trans: Vec<(ConstructorsBase, ConstructorsBase, bool)>,
    base: ConstructorsBase,
    failed: bool,
    ctx: Option<&'a Constructors>,
}
impl RenameConstr<'_> {
    #[allow(clippy::indexing_slicing)]
    fn push(&mut self, sig: &SigBuilder, i: SigId, allow: bool) {
        let lo = self.base;
        self.base += *sig.hi(i) - sig.sig[i].1;
        self.trans.push((lo, sig.sig[i].1, allow))
    }

    fn apply(&mut self, n: &mut u32, key: impl Fn(&ConstructorsBase) -> u32) {
        assert!(*n < key(&self.base));
        if let Some(i) = self
            .trans
            .partition_point(|p| key(&p.0) <= *n)
            .checked_sub(1)
        {
            #[allow(clippy::indexing_slicing)]
            let (from, to, allow) = &self.trans[i];
            *n = *n - key(from) + key(to);
            self.failed |= !*allow;
        }
    }

    fn ok(&mut self) -> bool {
        !std::mem::take(&mut self.failed)
    }
}

impl VisitMut for RenameConstr<'_> {
    const MODIFY_IDS: bool = true;
    fn abort(&self) -> bool {
        self.failed
    }
    fn visit_mode_id(&mut self, n: &mut ModeId) {
        self.apply(&mut n.0, |b| b.mode)
    }
    fn visit_struct_id(&mut self, n: &mut StructId) {
        self.apply(&mut n.0, |b| b.struct_mode)
    }
    fn visit_attr_id(&mut self, n: &mut AttrId) {
        self.apply(&mut n.0, |b| b.attribute)
    }
    fn visit_pred_id(&mut self, n: &mut PredId) {
        self.apply(&mut n.0, |b| b.predicate)
    }
    fn visit_func_id(&mut self, n: &mut FuncId) {
        self.apply(&mut n.0, |b| b.functor)
    }
    fn visit_sel_id(&mut self, n: &mut SelId) {
        self.apply(&mut n.0, |b| b.selector)
    }
    fn visit_aggr_id(&mut self, n: &mut AggrId) {
        self.apply(&mut n.0, |b| b.aggregate)
    }
    fn visit_attrs(&mut self, attrs: &mut Attrs) {
        if let Attrs::Consistent(attrs) = attrs {
            for attr in &mut *attrs {
                self.visit_attr_id(&mut attr.nr);
                self.visit_terms(&mut attr.args);
                if self.failed {
                    return;
                }
            }
            attrs.sort_by(|a, b| a.cmp(self.ctx, None, b, CmpStyle::Attr));
        }
    }
}

macro_rules! try_p {
    ($self:expr, $e:expr) => {
        Accomodator::report_parse_err(&mut $self.has_errors, $e)
    };
    ($self:expr, $pos:expr => $msg:expr, $e:expr) => {
        Accomodator::report_missing_warning(&mut $self.has_errors, $self.article, $pos, $e, {
            #[allow(unused)]
            use DirectiveKind::*;
            $msg
        })
    };
}

impl Accomodator {
    fn report_parse_err<T>(has_errors: &mut bool, val: PathResult<T>) -> Option<T> {
        match val {
            Ok(t) => Some(t),
            Err((path, e)) => {
                e.report(&path);
                *has_errors = true;
                None
            }
        }
    }
    fn report_missing_warning<T>(
        has_errors: &mut bool,
        art: Article,
        pos: Position,
        val: PathResult<T>,
        kind: DirectiveKind,
    ) -> Option<T> {
        match val {
            Ok(t) => Some(t),
            Err((path, ParseError::MissingFile)) => {
                report_accom_warning(kind, path, art, pos);
                None
            }
            Err((path, e)) => {
                e.report(&path);
                *has_errors = true;
                None
            }
        }
    }

    pub fn build_vocabularies(&self, vocs: &mut Vocabularies) {
        for (i, &(art, ref lo)) in self.dict.voc.enum_iter() {
            vocs.0.push((art, *self.dict.hi(i) - lo))
        }
    }

    /// ProcessVocabularies
    pub fn accom_symbols<'a>(
        &mut self,
        mml_vct: &'a [u8],
        syms: &mut Symbols,
        priority: &mut Vec<(PriorityKind, u32)>,
        mut infinitives: Option<&mut Vec<(PredSymId, &'a str)>>,
    ) {
        #[allow(clippy::indexing_slicing)]
        for &(_, art) in &self.dirs.0[DirectiveKind::Vocabularies] {
            let mut voc = Default::default();
            match art.read_vct(mml_vct, &mut voc) {
                Ok(true) => {}
                Ok(false) => {
                    // TODO: private vocabularies
                    println!(
            "error: {MML_VCT_PATH}: vocabulary for {art} not found (TODO: private vocabularies)"
          );
                    self.has_errors = true;
                    continue;
                }
                Err(e) => {
                    e.report(MML_VCT_PATH.as_ref());
                    self.has_errors = true;
                    continue;
                }
            }
            let hidden = self.dict.voc.is_empty();
            self.dict.voc.push((art, self.dict.base));
            if hidden {
                // The setup here is a bit weird. The HIDDEN article has some tokens defined
                // here by hard-coding, and other tokens defined in mml.vct. The net result is
                // that even though mml.vct says "G0 K0 L0 M1 O0 R2 U0 V1" it is really
                // treated as "G0 K3 L3 M2 O0 R3 U0 V1" when appearing in .dno files.
                for &(c, token) in SymbolData::BUILTIN_SYMBOLS {
                    let n = self.dict.base.0[c];
                    self.dict.base.0[c] += 1;
                    syms.push(((c, n).into(), token.to_owned()));
                }
            }
            for SymbolData { kind, token } in voc.symbols {
                let c = kind.class();
                let n = self.dict.base.0[c];
                self.dict.base.0[c] += 1;
                match kind {
                    SymbolDataKind::LeftBrk => {
                        priority.push((PriorityKind::LeftBrk(LeftBrkSymId(n)), DEFAULT_PRIO))
                    }
                    SymbolDataKind::RightBrk => {
                        priority.push((PriorityKind::RightBrk(RightBrkSymId(n)), DEFAULT_PRIO))
                    }
                    SymbolDataKind::Func { prio } => {
                        priority.push((PriorityKind::Functor(FuncSymId(n)), prio))
                    }
                    SymbolDataKind::Pred {
                        infinitive: Some(inf),
                    } => {
                        if let Some(infs) = &mut infinitives {
                            infs.push((PredSymId(n), inf))
                        }
                    }
                    _ => {}
                }
                syms.push(((c, n).into(), token.to_owned()))
            }
        }
    }

    #[allow(clippy::indexing_slicing)]
    pub fn accom_articles(&mut self) {
        let mut id = ArticleId(1); // 0 is reserved for SELF
        for &(_, art) in &self.dirs.0[DirectiveKind::Theorems] {
            assert!(self.articles.insert(art, id.fresh()).is_none())
        }
        for &(_, art) in &self.dirs.0[DirectiveKind::Schemes] {
            self.articles.entry(art).or_insert_with(|| id.fresh());
        }
    }

    /// ProcessConstructors
    #[allow(clippy::indexing_slicing)]
    pub fn accom_constructors(&mut self, constrs: &mut Constructors) -> io::Result<()> {
        for &(pos, art) in &self.dirs.0[DirectiveKind::Constructors] {
            let mut dco = Default::default();
            let result = MizPath { art }.read_dco(false, &mut dco, true);
            if try_p!(self, pos => Constructors, result).is_none() {
                continue;
            }
            for &art2 in &dco.sig {
                try_p!(self, self.sig.get_or_push(Some(constrs), art2));
            }
            if !self.sig.sig.0.iter().any(|p| p.0 == art) {
                self.sig.push_from(Some(constrs), art, &mut dco);
            }
        }
        Ok(())
    }

    /// ProcessRequirements
    #[allow(clippy::indexing_slicing)]
    pub fn accom_requirements(
        &mut self,
        ctx: &Constructors,
        idx: &mut RequirementIndexes,
    ) -> io::Result<()> {
        for &(pos, art) in &self.dirs.0[DirectiveKind::Requirements] {
            let mut dre = Default::default();
            if try_p!(self, pos => Requirements, MizPath { art }.read_dre(&mut dre)).is_none() {
                continue;
            }
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&dre.sig, Some(ctx))) else {
                continue;
            };
            for DepRequirement { req, mut kind } in dre.reqs {
                kind.visit(&mut rename);
                assert!(
                    rename.ok() && kind.lt(&ctx.len()),
                    "inaccessible requirement"
                );
                idx.set(req, kind);
            }
            self.sig.truncate(len);
        }
        Ok(())
    }

    /// ProcessClusters
    #[allow(clippy::indexing_slicing)]
    pub fn accom_clusters(
        &mut self,
        ctx: &Constructors,
        clusters: &mut Clusters,
    ) -> io::Result<()> {
        for &(_, art) in &self.dirs.0[DirectiveKind::Registrations] {
            let mut dcl = Default::default();
            let result = MizPath { art }.read_dcl(false, &mut dcl);
            let Some(_) = try_p!(self, catch_missing(result)) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&dcl.sig, Some(ctx))) else {
                continue;
            };
            for mut cl in dcl.cl.registered {
                cl.visit(&mut rename);
                if rename.ok() {
                    clusters.registered.push(cl);
                }
            }
            for mut cl in dcl.cl.functor {
                cl.visit(&mut rename);
                if rename.ok() {
                    clusters.functor.0.push(cl);
                }
            }
            for mut cl in dcl.cl.conditional {
                cl.visit(&mut rename);
                if rename.ok() {
                    clusters.conditional.push(ctx, cl);
                }
            }
            self.sig.truncate(len);
        }
        Ok(())
    }

    /// ProcessNotation
    #[allow(clippy::indexing_slicing)]
    pub fn accom_notations(
        &mut self,
        fmt_map: &mut HashMap<Format, FormatId>,
        mut fmts: Option<&mut Formats>,
        pats: &mut Vec<Pattern>,
    ) -> io::Result<()> {
        if let Some(fmts) = &mut fmts {
            assert_eq!(
                fmts.formats.push(Format::Attr(FormatAttr::STRICT)),
                FormatId::STRICT
            );
            fmt_map.insert(Format::Attr(FormatAttr::STRICT), FormatId::STRICT);
        }
        for &(pos, art) in &self.dirs.0[DirectiveKind::Notations] {
            let dict_len = self.dict.voc.len();
            let mut dno = Default::default();
            let result = MizPath { art }.read_dno(false, &mut dno);
            let Some(_) = try_p!(self, pos => Notations, result) else {
                continue;
            };
            let mut s_rename = RenameSymbol::default();
            for &(art, ref val) in &dno.vocs.0 {
                let i = self.dict.get_or_push(art, val);
                s_rename.push(val, &self.dict.voc[i].1, i.0 < dict_len as u32);
            }
            let sig_len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&dno.sig, None)) else {
                continue;
            };
            for Pattern {
                article,
                abs_nr,
                mut kind,
                mut fmt,
                mut primary,
                visible,
                pos,
            } in dno.pats
            {
                fmt.visit_mut(|k, c| s_rename.apply(k, c));
                if s_rename.ok() {
                    if let Some(fmt) = match &mut fmts {
                        Some(fmts) => {
                            Some(*fmt_map.entry(fmt).or_insert_with(|| fmts.formats.push(fmt)))
                        }
                        None => fmt_map.get(&fmt).copied(),
                    } {
                        kind.visit(&mut rename);
                        primary.visit(&mut rename);
                        if rename.ok() {
                            pats.push(Pattern {
                                article,
                                abs_nr,
                                kind,
                                fmt,
                                primary,
                                visible,
                                pos,
                            })
                        }
                    }
                }
            }
            self.dict.truncate(dict_len);
            self.sig.truncate(sig_len);
        }
        pats.sort_by_key(|pat| pat.kind.class() as u8);
        Ok(())
    }

    /// ProcessIdentify
    #[allow(clippy::indexing_slicing)]
    pub fn accom_identify_regs(
        &mut self,
        ctx: &Constructors,
        ids: &mut Vec<IdentifyFunc>,
    ) -> io::Result<()> {
        for &(_, art) in &self.dirs.0[DirectiveKind::Registrations] {
            let (mut sig, mut did) = Default::default();
            let result = MizPath { art }.read_did(false, &mut sig, &mut did);
            let Some(_) = try_p!(self, catch_missing(result)) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&sig, Some(ctx))) else {
                continue;
            };
            for mut id in did {
                id.visit(&mut rename);
                if rename.ok() {
                    ids.push(id);
                }
            }
            self.sig.truncate(len);
        }
        Ok(())
    }

    /// ProcessReductions
    #[allow(clippy::indexing_slicing)]
    pub fn accom_reduction_regs(
        &mut self,
        ctx: &Constructors,
        reds: &mut Vec<Reduction>,
    ) -> io::Result<()> {
        for &(_, art) in &self.dirs.0[DirectiveKind::Registrations] {
            let (mut sig, mut drd) = Default::default();
            let result = MizPath { art }.read_drd(false, &mut sig, &mut drd);
            let Some(_) = try_p!(self, catch_missing(result)) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&sig, Some(ctx))) else {
                continue;
            };
            for mut red in drd {
                red.visit(&mut rename);
                if rename.ok() {
                    reds.push(red);
                }
            }
            self.sig.truncate(len);
        }
        Ok(())
    }

    /// ProcessProperties
    #[allow(clippy::indexing_slicing)]
    pub fn accom_properties(
        &mut self,
        ctx: &Constructors,
        props: &mut Vec<Property>,
    ) -> io::Result<()> {
        for &(_, art) in &self.dirs.0[DirectiveKind::Registrations] {
            let (mut sig, mut dpr) = Default::default();
            let result = MizPath { art }.read_dpr(false, &mut sig, &mut dpr);
            let Some(_) = try_p!(self, catch_missing(result)) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&sig, Some(ctx))) else {
                continue;
            };
            for mut prop in dpr {
                prop.visit(&mut rename);
                if rename.ok() {
                    props.push(prop);
                }
            }
            self.sig.truncate(len);
        }
        Ok(())
    }

    /// ProcessDefinitions
    #[allow(clippy::indexing_slicing)]
    pub fn accom_definitions(
        &mut self,
        ctx: &Constructors,
        kind: DirectiveKind,
        defs: &mut Vec<Definiens>,
    ) -> io::Result<()> {
        for &(pos, art) in &self.dirs.0[kind] {
            let (mut sig, mut def) = Default::default();
            let result = MizPath { art }.read_def(false, &mut sig, &mut def);
            let Some(_) = try_p!(self, pos => kind, result) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&sig, Some(ctx))) else {
                continue;
            };
            for mut def in def {
                def.visit(&mut rename);
                if rename.ok() {
                    defs.push(def);
                }
            }
            self.sig.truncate(len);
        }
        Ok(())
    }

    /// ProcessTheorems
    #[allow(clippy::indexing_slicing)]
    pub fn accom_theorems(
        &mut self,
        write_eth: bool,
        ctx: &Constructors,
        def_map: &mut HashMap<DefRef, DefiniensId>,
        libs: &mut Libraries,
    ) -> io::Result<()> {
        let mut w = write_eth.then(|| MizPath { art: self.article }.write_eth());
        let mut defthms = DefiniensId::default();
        for &(pos, art) in &self.dirs.0[DirectiveKind::Theorems] {
            let mut thms = Default::default();
            let result = MizPath { art }.read_the(false, &mut thms);
            let Some(_) = try_p!(self, pos => Theorems, result) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&thms.sig, Some(ctx))) else {
                continue;
            };
            let (mut thm_nr, mut def_nr) = <(ThmId, DefId)>::default();
            let lib_nr = self.articles[&art];
            for mut thm in thms.thm {
                match thm.kind {
                    TheoremKind::Def(_) | TheoremKind::CanceledDef => {
                        let def_nr = def_nr.fresh();
                        thm.stmt.visit(&mut rename);
                        if rename.ok() {
                            if let Some(w) = &mut w {
                                w.write(lib_nr, def_nr.0, art, &thm)
                            }
                            libs.def.insert((lib_nr, def_nr), thm.stmt);
                            if let TheoremKind::Def(_) = thm.kind {
                                def_map.insert((lib_nr, def_nr), defthms.fresh());
                            }
                        }
                    }
                    TheoremKind::Thm | TheoremKind::CanceledThm => {
                        let thm_nr = thm_nr.fresh();
                        thm.stmt.visit(&mut rename);
                        if rename.ok() {
                            if let Some(w) = &mut w {
                                w.write(lib_nr, thm_nr.0, art, &thm)
                            }
                            libs.thm.insert((lib_nr, thm_nr), thm.stmt);
                        }
                    }
                }
            }
            self.sig.truncate(len);
        }
        if let Some(w) = w {
            w.finish()
        }
        Ok(())
    }

    /// ProcessSchemes
    #[allow(clippy::indexing_slicing)]
    pub fn accom_schemes(
        &mut self,
        write_esh: bool,
        ctx: &Constructors,
        libs: &mut Libraries,
    ) -> io::Result<()> {
        let mut w = write_esh.then(|| MizPath { art: self.article }.write_esh());
        for &(pos, art) in &self.dirs.0[DirectiveKind::Schemes] {
            let mut schs = Default::default();
            let result = MizPath { art }.read_sch(false, &mut schs);
            let Some(_) = try_p!(self, pos => Schemes, result) else {
                continue;
            };
            let len = self.sig.sig.len();
            let Some(mut rename) = try_p!(self, self.sig.rename(&schs.sig, Some(ctx))) else {
                continue;
            };
            let mut sch_nr = SchId::default();
            let lib_nr = self.articles[&art];
            for sch in schs.sch {
                let sch_nr = sch_nr.fresh();
                if let Some(mut sch) = sch {
                    sch.visit(&mut rename);
                    if rename.ok() {
                        if let Some(w) = &mut w {
                            w.write(lib_nr, sch_nr.0, art, &sch)
                        }
                        libs.sch.insert((lib_nr, sch_nr), sch);
                    }
                }
            }
            self.sig.truncate(len);
        }
        if let Some(w) = w {
            w.finish()
        }
        Ok(())
    }
}
