use std::mem::MaybeUninit;

use crate::{
    routes::Routes,
    storage::{HasJars, HasJarsDyn, JarFromJars},
};
use enum_index::full_map::EnumFullMap;
use husky_salsa_log_utils::HasLogger;

pub struct TestDb {
    storage: crate::Storage<Self>,
}

impl TestDb {
    pub fn new(
        initialize_jars: impl FnOnce(&mut <Self as HasJars>::Jars, &mut Routes<Self>),
    ) -> Self {
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

impl crate::database::AsSalsaDatabase for TestDb {
    fn as_salsa_database(&self) -> &dyn crate::Database {
        self
    }
}

impl crate::Database for TestDb {}

impl HasJars for TestDb {
    type Jars = TestJars;

    fn jars(&self) -> (&Self::Jars, &crate::Runtime) {
        self.storage.jars()
    }

    fn jars_mut(&mut self) -> (&mut Self::Jars, &mut crate::Runtime) {
        self.storage.jars_mut()
    }
}

// fn initialize_jars(jars: &mut Self::Jars, routes: &mut crate::routes::Routes<Self>) {
//     // just fill with zeros
//     *jars = TestJars {
//         map: EnumFullMap::new(|_| None),
//     };
//     // it's left for wrappers to complete the initialization
// }

impl HasJarsDyn for TestDb {
    fn runtime(&self) -> &crate::Runtime {
        self.storage.runtime()
    }
    fn runtime_mut(&mut self) -> &mut crate::Runtime {
        self.storage.runtime_mut()
    }
    fn maybe_changed_after(
        &self,
        input: crate::key::DependencyIndex,
        revision: crate::Revision,
    ) -> bool {
        let ingredient = self.storage.ingredient(input.ingredient_index());
        ingredient.maybe_changed_after(self, input, revision)
    }
    fn cycle_recovery_strategy(
        &self,
        ingredient_index: crate::IngredientIndex,
    ) -> crate::cycle::CycleRecoveryStrategy {
        let ingredient = self.storage.ingredient(ingredient_index);
        ingredient.cycle_recovery_strategy()
    }
    fn origin(
        &self,
        index: crate::DatabaseKeyIndex,
    ) -> Option<crate::runtime::local_state::QueryOrigin> {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.origin(index.key_index())
    }
    fn mark_validated_output(
        &self,
        executor: crate::DatabaseKeyIndex,
        output: crate::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(output.ingredient_index());
        ingredient.mark_validated_output(self, executor, output.key_index());
    }
    fn remove_stale_output(
        &self,
        executor: crate::DatabaseKeyIndex,
        stale_output: crate::key::DependencyIndex,
    ) {
        let ingredient = self.storage.ingredient(stale_output.ingredient_index());
        ingredient.remove_stale_output(self, executor, stale_output.key_index());
    }
    fn salsa_struct_deleted(&self, ingredient: crate::IngredientIndex, id: crate::Id) {
        let ingredient = self.storage.ingredient(ingredient);
        ingredient.salsa_struct_deleted(self, id);
    }
    fn fmt_index(
        &self,
        index: crate::key::DependencyIndex,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let ingredient = self.storage.ingredient(index.ingredient_index());
        ingredient.fmt_index(index.key_index(), fmt)
    }
}

impl<Jar> crate::storage::HasJar<Jar> for TestDb
where
    Jar: HasTestJarIndex + Sync + Send + 'static,
{
    fn jar(&self) -> (&Jar, &crate::Runtime) {
        let (jars, runtime) = self.jars();
        (Self::jar_from_jars(jars), runtime)
    }

    fn jar_mut(&mut self) -> (&mut Jar, &mut crate::Runtime) {
        let (jars, runtime) = self.jars_mut();
        (Self::jar_from_jars_mut(jars), runtime)
    }
}

impl<Jar> JarFromJars<Jar> for TestDb
where
    Jar: HasTestJarIndex + 'static,
{
    fn jar_from_jars(jars: &Self::Jars) -> &Jar {
        let any: &Box<dyn std::any::Any + Send + Sync + 'static> = jars.map
            [<Jar as HasTestJarIndex>::TEST_JAR_INDEX]
            .as_ref()
            .expect("should be initialized");
        let any: &(dyn std::any::Any + Send + Sync + 'static) = &**any;
        any.downcast_ref().expect("should be the right type")
    }

    fn jar_from_jars_mut(jars: &mut Self::Jars) -> &mut Jar {
        let any: &mut Box<dyn std::any::Any + Send + Sync + 'static> = jars.map
            [<Jar as HasTestJarIndex>::TEST_JAR_INDEX]
            .as_mut()
            .expect("should be initialized");
        let any: &mut (dyn std::any::Any + Send + Sync + 'static) = &mut **any;
        any.downcast_mut().expect("should be the right type")
    }
}

impl<Jar> crate::storage::DbWithJar<Jar> for TestDb
where
    Jar: crate::test_utils::HasTestJarIndex + Sync + Send + 'static,
{
    fn as_jar_db<'db>(&'db self) -> &'db <Jar as crate::jar::Jar<'db>>::DynDb
    where
        Jar: crate::jar::Jar<'db>,
    {
        Jar::cast_test_db(self)
    }
}
