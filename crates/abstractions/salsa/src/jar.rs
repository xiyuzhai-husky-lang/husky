use std::mem::MaybeUninit;

use enum_index::full_map::EnumFullVecMap;

use super::routes::Routes;

pub trait Jar: std::any::Any + HasJarIndex + Send + Sync + Sized + 'static {
    fn initialize(&mut self, routes: &mut Routes);
}

pub trait JarDyn: std::any::Any + Send + Sync + 'static {
    fn jar_index_dyn(&self) -> JarIndex;
    fn type_name(&self) -> &'static str;
}

impl<'a> dyn JarDyn + 'a {
    fn downcast_ref<Jar: 'static>(&self) -> Option<&Jar> {
        (self as &dyn std::any::Any).downcast_ref()
    }

    fn downcast_mut<Jar: 'static>(&mut self) -> Option<&mut Jar> {
        (self as &mut dyn std::any::Any).downcast_mut()
    }
}

impl<J> JarDyn for J
where
    J: Jar,
{
    fn jar_index_dyn(&self) -> JarIndex {
        J::JAR_INDEX
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }
}

#[derive(Default)]
pub struct Jars {
    map: EnumFullVecMap<JarIndex, Option<Box<dyn JarDyn>>>,
}

impl Jars {
    pub fn initialize_jar<Jar>(&mut self, routes: &mut Routes)
    where
        Jar: for<'db> crate::jar::Jar + HasJarIndex + Send + Sync + 'static,
    {
        let mut jar_maybe_uninitialized: MaybeUninit<Jar> = MaybeUninit::uninit();
        let jar: &mut Jar = unsafe { std::mem::transmute(&mut jar_maybe_uninitialized) };
        Jar::initialize(jar, routes);
        let index = <Jar as HasJarIndex>::JAR_INDEX;
        debug_assert!(self.map[index].is_none());
        self.map[index] =
            Some(unsafe { std::mem::transmute::<_, Box<Jar>>(Box::new(jar_maybe_uninitialized)) })
    }

    pub fn jar<Jar>(&self) -> &Jar
    where
        Jar: HasJarIndex + 'static,
    {
        let jar_index = <Jar as HasJarIndex>::JAR_INDEX;
        let Some(jar): Option<&Box<dyn JarDyn>> = self.map[jar_index].as_ref() else {
            unreachable!("{:?} should be initialized", jar_index)
        };
        let jar: &(dyn JarDyn) = &**jar;
        match jar.downcast_ref() {
            Some(jar) => jar,
            None => {
                eprintln!(
                    "jar_index = {:?}, expected = {:?}",
                    jar.jar_index_dyn(),
                    Jar::JAR_INDEX
                );
                eprintln!(
                    "jar.type_id() = {:?}, but expected = {:?}",
                    jar.type_id(),
                    std::any::TypeId::of::<Jar>()
                );
                eprintln!(
                    "jar.type_name() = `{}`, but expected = {:?}",
                    jar.type_name(),
                    std::any::type_name::<Jar>()
                );
                unreachable!("should be the right type");
            }
        }
    }

    pub fn jar_mut<Jar>(&mut self) -> &mut Jar
    where
        Jar: HasJarIndex + 'static,
    {
        let any: &mut Box<dyn JarDyn> = self.map[<Jar as HasJarIndex>::JAR_INDEX]
            .as_mut()
            .expect("should be initialized");
        let any: &mut (dyn JarDyn) = &mut **any;
        any.downcast_mut().expect("should be the right type")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, enum_index::IsEnumIndex)]
pub enum JarIndex {
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
    JavelinJar,
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

pub trait HasJarIndex {
    const JAR_INDEX: JarIndex;
}
