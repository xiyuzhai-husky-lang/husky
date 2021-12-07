//! Extracts, resolves and rewrites links and intra-doc links in markdown documentation.

mod intra_doc_links;

use common::*;

use either::Either;
use pulldown_cmark::{BrokenLink, CowStr, Event, InlineStr, LinkType, Options, Parser, Tag};
use pulldown_cmark_to_cmark::{cmark_with_options, Options as CMarkOptions};
use stdx::format_to;
use url::Url;

use hir::{db::HirDatabase, Adt, AsAssocItem, AssocItem, AssocItemContainer, Crate};
use ide_db::{
    defs::{Definition, NameClass, NameRefClass},
    helpers::pick_best_token,
    IdeDatabase,
};
use syntax::{ast, SyntaxKind::*, SyntaxNode, SyntaxToken};

use crate::{
    doc_links::intra_doc_links::{parse_intra_doc_link, strip_prefixes_suffixes},
    FilePosition, Semantics,
};

/// Weblink to an item's documentation.
pub(crate) type DocumentationLink = String;

const MARKDOWN_OPTIONS: Options = Options::ENABLE_FOOTNOTES
    .union(Options::ENABLE_TABLES)
    .union(Options::ENABLE_TASKLISTS);

/// Rewrite documentation links in markdown to point to an online host (e.g. docs.rs)
pub(crate) fn rewrite_links(db: &IdeDatabase, markdown: &str, definition: Definition) -> String {
    let mut cb = broken_link_clone_cb;
    let doc = Parser::new_with_broken_link_callback(markdown, MARKDOWN_OPTIONS, Some(&mut cb));

    let doc = map_links(doc, |target, title| {
        // This check is imperfect, there's some overlap between valid intra-doc links
        // and valid URLs so we choose to be too eager to try to resolve what might be
        // a URL.
        if target.contains("://") {
            (target.to_string(), title.to_string())
        } else {
            // Two possibilities:
            // * path-based links: `../../module/struct.MyStruct.html`
            // * module-based links (AKA intra-doc links): `super::super::module::MyStruct`
            if let Some(rewritten) = rewrite_intra_doc_link(db, definition, target, title) {
                return rewritten;
            }
            if let Some(target) = rewrite_url_link(db, definition, target) {
                return (target, title.to_string());
            }

            (target.to_string(), title.to_string())
        }
    });
    let mut out = String::new();
    cmark_with_options(
        doc,
        &mut out,
        None,
        CMarkOptions {
            code_block_backticks: 3,
            ..Default::default()
        },
    )
    .ok();
    out
}

/// Remove all links in markdown documentation.
pub(crate) fn remove_links(markdown: &str) -> String {
    let mut drop_link = false;

    let mut cb = |_: BrokenLink| {
        let empty = InlineStr::try_from("").unwrap();
        Some((CowStr::Inlined(empty), CowStr::Inlined(empty)))
    };
    let doc = Parser::new_with_broken_link_callback(markdown, MARKDOWN_OPTIONS, Some(&mut cb));
    let doc = doc.filter_map(move |evt| match evt {
        Event::Start(Tag::Link(link_type, target, title)) => {
            if link_type == LinkType::Inline && target.contains("://") {
                Some(Event::Start(Tag::Link(link_type, target, title)))
            } else {
                drop_link = true;
                None
            }
        }
        Event::End(_) if drop_link => {
            drop_link = false;
            None
        }
        _ => Some(evt),
    });

    let mut out = String::new();
    cmark_with_options(
        doc,
        &mut out,
        None,
        CMarkOptions {
            code_block_backticks: 3,
            ..Default::default()
        },
    )
    .ok();
    out
}

/// Retrieve a link to documentation for the given symbol.
pub(crate) fn external_docs(
    db: &IdeDatabase,
    position: &FilePosition,
) -> Option<DocumentationLink> {
    todo!()
}

/// Extracts all links from a given markdown text returning the definition text range, link-text
/// and the namespace if known.
pub(crate) fn extract_definitions_from_docs(
    docs: &hir::Documentation,
) -> Vec<(TextRange, String, Option<hir::Namespace>)> {
    todo!()
}

pub(crate) fn resolve_doc_path_for_def(
    db: &dyn HirDatabase,
    def: Definition,
    link: &str,
    ns: Option<hir::Namespace>,
) -> Option<Definition> {
    todo!()
}

pub(crate) fn doc_attributes(
    sema: &Semantics<IdeDatabase>,
    node: &SyntaxNode,
) -> Option<(hir::AttrsWithOwner, Definition)> {
    todo!()
}

pub(crate) struct DocCommentToken {
    doc_token: SyntaxToken,
    prefix_len: TextSize,
}

pub(crate) fn token_as_doc_comment(doc_token: &SyntaxToken) -> Option<DocCommentToken> {
    todo!()
}

impl DocCommentToken {
    pub(crate) fn get_definition_with_descend_at<T>(
        self,
        sema: &Semantics<IdeDatabase>,
        offset: TextSize,
        // Definition, CommentOwner, range of intra doc link in original file
        mut cb: impl FnMut(Definition, SyntaxNode, TextRange) -> Option<T>,
    ) -> Option<T> {
        todo!()
    }
}

fn broken_link_clone_cb<'a, 'b>(link: BrokenLink<'a>) -> Option<(CowStr<'b>, CowStr<'b>)> {
    // These allocations are actually unnecessary but the lifetimes on BrokenLinkCallback are wrong
    // this is fixed in the repo but not on the crates.io release yet
    Some((
        /*url*/ link.reference.to_owned().into(),
        /*title*/ link.reference.to_owned().into(),
    ))
}

// FIXME:
// BUG: For Option::Some
// Returns https://doc.rust-lang.org/nightly/core/prelude/v1/enum.Option.html#variant.Some
// Instead of https://doc.rust-lang.org/nightly/core/option/enum.Option.html
//
// This should cease to be a problem if RFC2988 (Stable Rustdoc URLs) is implemented
// https://github.com/rust-lang/rfcs/pull/2988
fn get_doc_link(db: &IdeDatabase, def: Definition) -> Option<String> {
    todo!()
    // let (target, file, frag) = filename_and_frag_for_def(db, def)?;

    // let krate = crate_of_def(db, target)?;
    // let mut url = get_doc_base_url(db, &krate)?;

    // if let Some(path) = mod_path_of_def(db, target) {
    //     url = url.join(&path).ok()?;
    // }

    // url = url.join(&file).ok()?;
    // url.set_fragment(frag.as_deref());

    // Some(url.into())
}

fn rewrite_intra_doc_link(
    db: &IdeDatabase,
    def: Definition,
    target: &str,
    title: &str,
) -> Option<(String, String)> {
    todo!()

    // let (link, ns) = parse_intra_doc_link(target);

    // let resolved = resolve_doc_path_for_def(db, def, link, ns)?;
    // let krate = crate_of_def(db, resolved)?;
    // let mut url = get_doc_base_url(db, &krate)?;

    // let (_, file, frag) = filename_and_frag_for_def(db, resolved)?;
    // if let Some(path) = mod_path_of_def(db, resolved) {
    //     url = url.join(&path).ok()?;
    // }

    // url = url.join(&file).ok()?;
    // url.set_fragment(frag.as_deref());

    // Some((url.into(), strip_prefixes_suffixes(title).to_string()))
}

/// Try to resolve path to local documentation via path-based links (i.e. `../gateway/struct.Shard.html`).
fn rewrite_url_link(db: &IdeDatabase, def: Definition, target: &str) -> Option<String> {
    todo!()
    // if !(target.contains('#') || target.contains(".html")) {
    //     return None;
    // }

    // let krate = crate_of_def(db, def)?;
    // let mut url = get_doc_base_url(db, &krate)?;
    // let (def, file, frag) = filename_and_frag_for_def(db, def)?;

    // if let Some(path) = mod_path_of_def(db, def) {
    //     url = url.join(&path).ok()?;
    // }

    // url = url.join(&file).ok()?;
    // url.set_fragment(frag.as_deref());
    // url.join(target).ok().map(Into::into)
}

fn crate_of_def(db: &IdeDatabase, def: Definition) -> Option<Crate> {
    todo!()
}

fn mod_path_of_def(db: &IdeDatabase, def: Definition) -> Option<String> {
    todo!()
}

/// Rewrites a markdown document, applying 'callback' to each link.
fn map_links<'e>(
    events: impl Iterator<Item = Event<'e>>,
    callback: impl Fn(&str, &str) -> (String, String),
) -> impl Iterator<Item = Event<'e>> {
    let mut in_link = false;
    let mut link_target: Option<CowStr> = None;

    events.map(move |evt| match evt {
        Event::Start(Tag::Link(_, ref target, _)) => {
            in_link = true;
            link_target = Some(target.clone());
            evt
        }
        Event::End(Tag::Link(link_type, target, _)) => {
            in_link = false;
            Event::End(Tag::Link(
                link_type,
                link_target.take().unwrap_or(target),
                CowStr::Borrowed(""),
            ))
        }
        Event::Text(s) if in_link => {
            let (link_target_s, link_name) = callback(&link_target.take().unwrap(), &s);
            link_target = Some(CowStr::Boxed(link_target_s.into()));
            Event::Text(CowStr::Boxed(link_name.into()))
        }
        Event::Code(s) if in_link => {
            let (link_target_s, link_name) = callback(&link_target.take().unwrap(), &s);
            link_target = Some(CowStr::Boxed(link_target_s.into()));
            Event::Code(CowStr::Boxed(link_name.into()))
        }
        _ => evt,
    })
}

/// Get the root URL for the documentation of a crate.
///
/// ```ignore
/// https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
fn get_doc_base_url(db: &IdeDatabase, krate: &Crate) -> Option<Url> {
    todo!()
}

/// Get the filename and extension generated for a symbol by rustdoc.
///
/// ```ignore
/// https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
///                                    ^^^^^^^^^^^^^^^^^^^
/// ```
fn filename_and_frag_for_def(
    db: &dyn HirDatabase,
    def: Definition,
) -> Option<(Definition, String, Option<String>)> {
    todo!()
}

/// Get the fragment required to link to a specific field, method, associated type, or associated constant.
///
/// ```ignore
/// https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
///                                                       ^^^^^^^^^^^^^^
/// ```
fn get_assoc_item_fragment(db: &dyn HirDatabase, assoc_item: hir::AssocItem) -> Option<String> {
    todo!()
}
