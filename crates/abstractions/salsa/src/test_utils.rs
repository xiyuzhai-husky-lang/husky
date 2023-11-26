use crate::routes::Routes;
use crate::*;
use enum_index::full_map::EnumFullMap;
use husky_salsa_log_utils::HasLogger;
use std::mem::MaybeUninit;

pub struct TestDb {
    storage: crate::Storage,
}

impl TestDb {
    /// here we use fn instead of impl FnOnce to save compilation time
    pub fn new(initialize_jars: fn(&mut Jars, &mut Routes<Self>)) -> Self {
        Self {
            storage: crate::Storage::new(initialize_jars),
        }
    }
}

impl HasLogger for TestDb {
    fn logger(&self) -> &husky_salsa_log_utils::Logger {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, enum_index::IsEnumIndex)]
pub enum TestJarIndex {
    Jar,
    // comptime
    InstructionJar,
    RustTranspilationJar,
    // devtime
    TraceJar,
    // fs
    VfsJar,
    // hir
    HirDeclJar,
    HirDefnJar,
    HirEagerExprJar,
    HirExprJar,
    HirLazyExprJar,
    HirPreludeJar,
    HirTypeJar,
    // ide
    CompletionJar,
    DiagnosticsJar,
    DocumentationJar,
    FoldingRangeJar,
    HoverJar,
    SemanticTokenJar,
    SyntaxFormatJar,
    TokenInfoJar,
    // kernel
    CowordJar,
    DeclarativeSignatureJar,
    DeclarativeTermJar,
    DeclarativeTypeJar,
    EntityPathJar,
    EtherealSignatureJar,
    EtherealTermJar,
    FluffyTermJar,
    TermPreludeJar,
    // lex
    TextJar,
    TokenJar,
    TokenDataJar,
    TomlTokenJar,
    // linkage
    LinkageJar,
    // semantics
    CorgiConfigJar,
    ManifestJar,
    ToolchainConfigJar,
    SemaExprJar,
    // syntax
    AstJar,
    EntitySynTreeJar,
    ManifestAstJar,
    SynDeclJar,
    SynDefnJar,
    SynExprJar,
    TomlAstJar,
    CorgiConfigAstJar,
    // val
    ValJar,
    ValReprJar,
}

#[cfg(debug_assertions)]
pub trait HasTestJarIndex {
    const TEST_JAR_INDEX: TestJarIndex;
}

#[derive(Default)]
pub struct TestJars {
    map: EnumFullMap<TestJarIndex, Option<Box<dyn std::any::Any + Sync + Send>>>,
}

impl TestJars {
    pub fn initialize_jar<Jar>(&mut self, routes: &mut Routes<TestDb>)
    where
        Jar: for<'db> crate::jar::Jar<'db> + HasTestJarIndex + Send + Sync + 'static,
    {
        let mut jar_maybe_uninitialized: MaybeUninit<Jar> = MaybeUninit::uninit();
        let jar: &mut Jar = unsafe { std::mem::transmute(&mut jar_maybe_uninitialized) };
        Jar::initialize(jar, routes);
        let index = <Jar as HasTestJarIndex>::TEST_JAR_INDEX;
        debug_assert!(self.map[index].is_none());
        self.map[index] =
            Some(unsafe { std::mem::transmute::<_, Box<Jar>>(Box::new(jar_maybe_uninitialized)) })
    }
}
