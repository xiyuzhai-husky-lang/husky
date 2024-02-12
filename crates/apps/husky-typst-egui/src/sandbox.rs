use std::sync::Arc;

use comemo::Prehashed;
use husky_tex::{diag::FileResult, syntax::VirtualPath, LibraryBuilder};
use husky_tex::{
    foundations::Bytes,
    syntax::{FileId, Source},
};
use husky_tex::{foundations::Datetime, Library};
use husky_tex::{
    text::{TexFont, TexFontBook},
    IsTexWorld,
};

pub struct Sandbox {
    library: Prehashed<Library>,
    book: Prehashed<TexFontBook>,
    fonts: Vec<TexFont>,
}

fn make_source(source: String) -> Source {
    Source::new(FileId::new_fake(VirtualPath::new("input.typ")), source)
}

fn fonts() -> Vec<TexFont> {
    use husky_path_utils::rust::husky_cargo_workspace_manifest_dir;

    std::fs::read_dir(husky_cargo_workspace_manifest_dir().join("assets/fonts"))
        .unwrap()
        .map(Result::unwrap)
        .flat_map(|entry| {
            let bytes = std::fs::read(entry.path()).unwrap();
            let buffer = Bytes::from(bytes);
            TexFont::iter(buffer)
        })
        .collect()
}

pub struct WithSource {
    sandbox: Arc<Sandbox>,
    source: Source,
}

impl Sandbox {
    pub fn new() -> Self {
        let fonts = fonts();

        Self {
            library: Prehashed::new(LibraryBuilder::default().build()),
            book: Prehashed::new(TexFontBook::from_fonts(&fonts)),
            fonts,
        }
    }

    pub fn with_source(self: Arc<Self>, source: String) -> WithSource {
        WithSource {
            sandbox: self,
            source: make_source(source),
        }
    }
}

impl WithSource {
    pub fn into_source(self) -> Source {
        self.source
    }
}

impl IsTexWorld for WithSource {
    fn library(&self) -> &Prehashed<Library> {
        &self.sandbox.library
    }

    fn main(&self) -> Source {
        self.source.clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        assert_eq!(id, self.source.id());
        Ok(self.source.clone())
    }

    fn book(&self) -> &Prehashed<TexFontBook> {
        &self.sandbox.book
    }

    fn font(&self, id: usize) -> Option<TexFont> {
        self.sandbox.fonts.get(id).cloned()
    }

    fn file(&self, path: FileId) -> FileResult<Bytes> {
        // Err(FileError::NotFound(path.into()))
        todo!()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        todo!()
    }
}

#[test]
fn sandbox_works() {
    use expect_test::expect;
    use husky_tex::compile;
    use husky_tex::eval::Tracer;

    let sandbox = Sandbox::new();
    let mut tracer = Tracer::new();
    let source = r#"hello world

$ x + x $
"#
    .to_string();
    expect![[r#"
        Ok(
            TexDocument {
                pages: [
                    Page {
                        frame: Frame [
                            Text("hello world"),
                            Group Frame [
                                Elem(Element(equation-tex)),
                                Elem(Element(equation-tex)),
                                Elem(Element(equation-tex)),
                                Text("𝑥"),
                                Elem(Element(equation-tex)),
                                Text("+"),
                                Elem(Element(equation-tex)),
                                Text("𝑥"),
                            ],
                            Elem(Element(equation-tex)),
                        ],
                        numbering: None,
                        number: 1,
                    },
                ],
                title: None,
                author: [],
                keywords: [],
                date: Auto,
                introspector: Introspector(..),
            },
        )
    "#]]
    .assert_debug_eq(&compile(
        &Arc::new(sandbox).with_source(source),
        &mut tracer,
    ));
}
