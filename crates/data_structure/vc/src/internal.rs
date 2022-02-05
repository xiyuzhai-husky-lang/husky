use std::{collections::HashMap, hash::Hash, sync::Arc};

use common::*;

use crate::*;

#[derive(Debug)]
pub struct VersionControlInternal<NameId, T>
where
    NameId: Hash + Copy + Eq,
    T: IssuerData,
{
    next_uid_raw: usize,
    entry_indices: HashMap<NameId, usize>,
    entries: Vec<VersionControlEntry<T>>,
}

impl<NameId, T> VersionControlInternal<NameId, T>
where
    NameId: Hash + Copy + Eq,
    T: IssuerData,
{
    pub(crate) fn new() -> Self {
        Self {
            next_uid_raw: 0,
            entry_indices: Default::default(),
            entries: vec![],
        }
    }

    pub(crate) fn uid(&mut self, name: NameId) -> Uid {
        let idx = self.idx(&name);
        self.entries[idx].uid
    }

    pub(crate) fn update(&mut self, name: NameId, data: Arc<T>, dependees: &[NameId]) -> Uid {
        let idx = self.idx(&name);
        if let Some(ref prev_data) = self.entries[idx].data {
            if **prev_data != *data {
                self.entries[idx].uid = self.new_uid();
                self.update_dependants(idx);
            }
        }
        self.entries[idx].data = Some(data);
        let uid = self.entries[idx].uid;
        for dependee in dependees
            .iter()
            .map(|name| self.idx(name))
            .collect::<Vec<usize>>()
        {
            self.subscribe(Dependant { idx, uid }, dependee);
        }
        uid
    }

    fn new_uid(&mut self) -> Uid {
        let uid = Uid {
            raw: self.next_uid_raw,
        };
        self.next_uid_raw += 1;
        uid
    }

    fn idx(&mut self, name: &NameId) -> usize {
        if let Some(idx) = self.entry_indices.get(name).map(|idx| *idx) {
            idx
        } else {
            let idx = self.entries.len();
            self.entry_indices.insert(*name, idx);
            let new_uid = self.new_uid();
            self.entries.push(VersionControlEntry {
                data: None,
                uid: new_uid,
                dependants: vec![],
            });
            idx
        }
    }

    fn subscribe(&mut self, dependant: Dependant, dependee: usize) {
        self.entries[dependee].dependants.push(dependant);
    }

    fn update_dependants(&mut self, dependee: usize) {
        for dependant in std::mem::take(&mut self.entries[dependee].dependants) {
            if self.entries[dependant.idx].uid == dependant.uid {
                let new_uid = self.new_uid();
                self.entries[dependant.idx].uid = new_uid;
                self.subscribe(
                    Dependant {
                        idx: dependant.idx,
                        uid: new_uid,
                    },
                    dependee,
                );
                self.update_dependants(dependant.idx)
            }
        }
    }
}

pub struct VersionControlEntry<T>
where
    T: IssuerData,
{
    data: Option<Arc<T>>,
    uid: Uid,
    dependants: Vec<Dependant>,
}

impl<T> std::fmt::Debug for VersionControlEntry<T>
where
    T: IssuerData,
{
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Entry {{uid: {:?}, dependants: {:?}, data: {:?}}}",
            &self.uid, &self.dependants, &self.data
        ))
    }
}

#[derive(Clone, Copy)]
pub struct Dependant {
    idx: usize,
    uid: Uid,
}

impl std::fmt::Debug for Dependant {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}({})", self.idx, self.uid.raw))
    }
}
