use crate::engine::sequential::{dfs::GacDfsSequentialEngine, IsGacSequentialEngineInner};
use either::*;
use eterned::db::EternerDb;
use rustc_hash::{FxHashMap, FxHashSet};

// # node

#[eterned::eterned]
pub struct StringPrefixTrieNode {
    pub parent: Option<StringPrefixTrieNode>,
    pub char: char,
}

impl std::fmt::Debug for StringPrefixTrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// # state

#[derive(Copy, Clone)]
pub struct StringPrefixTrieState {
    cursor: usize,
}

// # session

pub struct StringPrefixTrieEngineSession {
    all_nodes: Vec<StringPrefixTrieNode>,
    root_nodes: Vec<StringPrefixTrieNode>,
    children_map: FxHashMap<StringPrefixTrieNode, FxHashSet<StringPrefixTrieNode>>,
    terminal_nodes: FxHashSet<StringPrefixTrieNode>,
    input: Vec<char>,
}

impl StringPrefixTrieEngineSession {
    pub fn new(entries: Vec<String>, input: String, db: &EternerDb) -> Self {
        let mut all_nodes = Vec::new();
        let mut root_nodes = Vec::new();
        let mut children_map: FxHashMap<StringPrefixTrieNode, FxHashSet<StringPrefixTrieNode>> =
            FxHashMap::default();
        let mut terminal_nodes = FxHashSet::default();
        for entry in entries {
            let mut chars = entry.chars();
            let c = chars.next().unwrap();
            let mut node = StringPrefixTrieNode::new(None, c, db);
            root_nodes.push(node);
            for c in chars {
                let mut child = StringPrefixTrieNode::new(Some(node), c, db);
                children_map.entry(node).or_default().insert(child);
                node = child;
            }
            terminal_nodes.insert(node);
        }
        Self {
            all_nodes,
            root_nodes,
            children_map,
            terminal_nodes,
            input: input.chars().collect(),
        }
    }
}

//# engine

pub struct StringPrefixTrieEngineInner<'sess> {
    session: &'sess StringPrefixTrieEngineSession,
}

impl<'sess> IsGacSequentialEngineInner<'sess> for StringPrefixTrieEngineInner<'sess> {
    type Node = StringPrefixTrieNode;
    type Input = ();
    type State = StringPrefixTrieState;
    type Output = StringPrefixTrieNode;

    fn roots(&self) -> impl IntoIterator<Item = Self::Node> + 'sess {
        self.session.root_nodes.iter().copied()
    }

    fn children(&self, node: Self::Node) -> impl IntoIterator<Item = Self::Node> + 'sess {
        self.session.children_map[&node].iter().copied()
    }

    fn initial_state(&self, input: Self::Input) -> Self::State {
        StringPrefixTrieState { cursor: 0 }
    }

    fn process(
        &mut self,
        node: Self::Node,
        state: Self::State,
    ) -> Either<Option<Self::State>, Self::Output> {
        if node.char() == self.session.input[state.cursor] {
            if self.session.terminal_nodes.contains(&node) {
                Right(node)
            } else {
                Left(Some(StringPrefixTrieState {
                    cursor: state.cursor + 1,
                }))
            }
        } else {
            Left(None)
        }
    }
}

#[test]
fn string_prefix_trie_with_gac_dfs_sequential_engine_works() {
    fn t(db: &EternerDb, entries: &[&str], input: &str) -> bool {
        let session = StringPrefixTrieEngineSession::new(
            entries.iter().map(|s| s.to_string()).collect(),
            input.to_string(),
            db,
        );
        let engine = GacDfsSequentialEngine::new(StringPrefixTrieEngineInner { session: &session });
        engine.search(()).is_some()
    }

    let db = EternerDb::default();
    assert!(t(&db, &["a"], "a"));
    assert!(!t(&db, &["a"], "b"));
    assert!(t(&db, &["a", "ab"], "abc"));
    assert!(t(&db, &["a", "ab"], "abcd"));
}
