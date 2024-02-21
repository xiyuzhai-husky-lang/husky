use std::collections::HashSet;
use std::ops::Range;

use ecow::{eco_format, EcoString};
use unicode_math_class::MathClass;

use crate::{ast, is_ident, is_newline, LexMode, Lexer, TypstSyntaxKind, TypstSyntaxNode};

/// Parse a source file.
pub fn parse(text: &str) -> TypstSyntaxNode {
    let mut p = Parser::new(text, 0, LexMode::TypstMarkup);
    markup(&mut p, true, 0, |_| false);
    p.finish().into_iter().next().unwrap()
}

/// Parse top-level code.
pub fn parse_code(text: &str) -> TypstSyntaxNode {
    let mut p = Parser::new(text, 0, LexMode::Code);
    let m = p.marker();
    p.skip();
    code_exprs(&mut p, |_| false);
    p.wrap_all(m, TypstSyntaxKind::Code);
    p.finish().into_iter().next().unwrap()
}

/// Parse top-level math.
pub fn parse_math(text: &str) -> TypstSyntaxNode {
    let mut p = Parser::new(text, 0, LexMode::Math);
    math(&mut p, |_| false);
    p.finish().into_iter().next().unwrap()
}

fn markup(
    p: &mut Parser,
    mut at_start: bool,
    min_indent: usize,
    mut stop: impl FnMut(&Parser) -> bool,
) {
    let m = p.marker();
    let mut nesting: usize = 0;
    while !p.eof() {
        match p.current() {
            TypstSyntaxKind::LeftDelimiter => nesting += 1,
            TypstSyntaxKind::RightDelimiter if nesting > 0 => nesting -= 1,
            _ if stop(p) => break,
            _ => {}
        }

        if p.newline() {
            at_start = true;
            if min_indent > 0 && p.column(p.current_end()) < min_indent {
                break;
            }
            p.eat();
            continue;
        }

        let prev = p.prev_end();
        markup_expr(p, &mut at_start);
        if !p.progress(prev) {
            p.unexpected();
        }
    }
    p.wrap(m, TypstSyntaxKind::TypstMarkup);
}

pub(super) fn reparse_markup(
    text: &str,
    range: Range<usize>,
    at_start: &mut bool,
    nesting: &mut usize,
    mut stop: impl FnMut(TypstSyntaxKind) -> bool,
) -> Option<Vec<TypstSyntaxNode>> {
    let mut p = Parser::new(text, range.start, LexMode::TypstMarkup);
    while !p.eof() && p.current_start() < range.end {
        match p.current() {
            TypstSyntaxKind::LeftDelimiter => *nesting += 1,
            TypstSyntaxKind::RightDelimiter if *nesting > 0 => *nesting -= 1,
            _ if stop(p.current()) => break,
            _ => {}
        }

        if p.newline() {
            *at_start = true;
            p.eat();
            continue;
        }

        let prev = p.prev_end();
        markup_expr(&mut p, at_start);
        if !p.progress(prev) {
            p.unexpected();
        }
    }
    (p.balanced && p.current_start() == range.end).then(|| p.finish())
}

fn markup_expr(p: &mut Parser, at_start: &mut bool) {
    match p.current() {
        TypstSyntaxKind::Space
        | TypstSyntaxKind::Parbreak
        | TypstSyntaxKind::LineComment
        | TypstSyntaxKind::BlockComment => {
            p.eat();
            return;
        }

        TypstSyntaxKind::Text
        | TypstSyntaxKind::Linebreak
        | TypstSyntaxKind::Escape
        | TypstSyntaxKind::Shorthand
        | TypstSyntaxKind::SmartQuote
        | TypstSyntaxKind::Raw
        | TypstSyntaxKind::Link
        | TypstSyntaxKind::Label => p.eat(),

        TypstSyntaxKind::Hash => embedded_code_expr(p),
        TypstSyntaxKind::Star => strong(p),
        TypstSyntaxKind::Underscore => emph(p),
        TypstSyntaxKind::HeadingMarker if *at_start => heading(p),
        TypstSyntaxKind::ListMarker if *at_start => list_item(p),
        TypstSyntaxKind::EnumMarker if *at_start => enum_item(p),
        TypstSyntaxKind::TermMarker if *at_start => term_item(p),
        TypstSyntaxKind::RefMarker => reference(p),
        TypstSyntaxKind::Dollar => equation(p),

        TypstSyntaxKind::LeftDelimiter
        | TypstSyntaxKind::RightDelimiter
        | TypstSyntaxKind::HeadingMarker
        | TypstSyntaxKind::ListMarker
        | TypstSyntaxKind::EnumMarker
        | TypstSyntaxKind::TermMarker
        | TypstSyntaxKind::Colon => p.convert(TypstSyntaxKind::Text),

        _ => {}
    }

    *at_start = false;
}

fn strong(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Star);
    markup(p, false, 0, |p| {
        p.at(TypstSyntaxKind::Star)
            || p.at(TypstSyntaxKind::Parbreak)
            || p.at(TypstSyntaxKind::RightDelimiter)
    });
    p.expect_closing_delimiter(m, TypstSyntaxKind::Star);
    p.wrap(m, TypstSyntaxKind::Strong);
}

fn emph(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Underscore);
    markup(p, false, 0, |p| {
        p.at(TypstSyntaxKind::Underscore)
            || p.at(TypstSyntaxKind::Parbreak)
            || p.at(TypstSyntaxKind::RightDelimiter)
    });
    p.expect_closing_delimiter(m, TypstSyntaxKind::Underscore);
    p.wrap(m, TypstSyntaxKind::Emph);
}

fn heading(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::HeadingMarker);
    whitespace_line(p);
    markup(p, false, usize::MAX, |p| {
        p.at(TypstSyntaxKind::Label)
            || p.at(TypstSyntaxKind::RightDelimiter)
            || (p.at(TypstSyntaxKind::Space) && p.lexer.clone().next() == TypstSyntaxKind::Label)
    });
    p.wrap(m, TypstSyntaxKind::Heading);
}

fn list_item(p: &mut Parser) {
    let m = p.marker();
    let min_indent = p.column(p.current_start()) + 1;
    p.assert(TypstSyntaxKind::ListMarker);
    whitespace_line(p);
    markup(p, false, min_indent, |p| {
        p.at(TypstSyntaxKind::RightDelimiter)
    });
    p.wrap(m, TypstSyntaxKind::ListItem);
}

fn enum_item(p: &mut Parser) {
    let m = p.marker();
    let min_indent = p.column(p.current_start()) + 1;
    p.assert(TypstSyntaxKind::EnumMarker);
    whitespace_line(p);
    markup(p, false, min_indent, |p| {
        p.at(TypstSyntaxKind::RightDelimiter)
    });
    p.wrap(m, TypstSyntaxKind::EnumItem);
}

fn term_item(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::TermMarker);
    let min_indent = p.column(p.prev_end());
    whitespace_line(p);
    markup(p, false, usize::MAX, |p| {
        p.at(TypstSyntaxKind::Colon) || p.at(TypstSyntaxKind::RightDelimiter)
    });
    p.expect(TypstSyntaxKind::Colon);
    whitespace_line(p);
    markup(p, false, min_indent, |p| {
        p.at(TypstSyntaxKind::RightDelimiter)
    });
    p.wrap(m, TypstSyntaxKind::TermItem);
}

fn reference(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::RefMarker);
    if p.directly_at(TypstSyntaxKind::LeftDelimiter) {
        content_block(p);
    }
    p.wrap(m, TypstSyntaxKind::Ref);
}

fn whitespace_line(p: &mut Parser) {
    while !p.newline() && p.current().is_trivia() {
        p.eat();
    }
}

fn equation(p: &mut Parser) {
    let m = p.marker();
    p.enter(LexMode::Math);
    p.assert(TypstSyntaxKind::Dollar);
    math(p, |p| p.at(TypstSyntaxKind::Dollar));
    p.expect_closing_delimiter(m, TypstSyntaxKind::Dollar);
    p.exit();
    p.wrap(m, TypstSyntaxKind::Equation);
}

fn math(p: &mut Parser, mut stop: impl FnMut(&Parser) -> bool) {
    let m = p.marker();
    while !p.eof() && !stop(p) {
        let prev = p.prev_end();
        math_expr(p);
        if !p.progress(prev) {
            p.unexpected();
        }
    }
    p.wrap(m, TypstSyntaxKind::Math);
}

fn math_expr(p: &mut Parser) {
    math_expr_prec(p, 0, TypstSyntaxKind::Eof)
}

fn math_expr_prec(p: &mut Parser, min_prec: usize, stop: TypstSyntaxKind) {
    let m = p.marker();
    let mut continuable = false;
    match p.current() {
        TypstSyntaxKind::Hash => embedded_code_expr(p),
        TypstSyntaxKind::MathIdent => {
            continuable = true;
            p.eat();
            while p.directly_at(TypstSyntaxKind::Text) && p.current_text() == "." && {
                let mut copy = p.lexer.clone();
                let start = copy.cursor();
                let next = copy.next();
                let end = copy.cursor();
                matches!(next, TypstSyntaxKind::MathIdent | TypstSyntaxKind::Text)
                    && is_ident(&p.text[start..end])
            } {
                p.convert(TypstSyntaxKind::Dot);
                p.convert(TypstSyntaxKind::Ident);
                p.wrap(m, TypstSyntaxKind::FieldAccess);
            }
            if min_prec < 3 && p.directly_at(TypstSyntaxKind::Text) && p.current_text() == "(" {
                math_args(p);
                p.wrap(m, TypstSyntaxKind::FuncCall);
                continuable = false;
            }
        }

        TypstSyntaxKind::Text | TypstSyntaxKind::Shorthand => {
            continuable = matches!(
                math_class(p.current_text()),
                None | Some(MathClass::Alphabetic)
            );
            if !maybe_delimited(p) {
                p.eat();
            }
        }

        TypstSyntaxKind::Linebreak | TypstSyntaxKind::MathAlignPoint => p.eat(),
        TypstSyntaxKind::Escape | TypstSyntaxKind::Str => {
            continuable = true;
            p.eat();
        }

        TypstSyntaxKind::Root => {
            if min_prec < 3 {
                p.eat();
                let m2 = p.marker();
                math_expr_prec(p, 2, stop);
                math_unparen(p, m2);
                p.wrap(m, TypstSyntaxKind::MathRoot);
            }
        }

        TypstSyntaxKind::Prime => {
            // Means that there is nothing to attach the prime to.
            continuable = true;
            while p.at(TypstSyntaxKind::Prime) {
                let m2 = p.marker();
                p.eat();
                // Eat the group until the space.
                while p.eat_if_direct(TypstSyntaxKind::Prime) {}
                p.wrap(m2, TypstSyntaxKind::MathPrimes);
            }
        }

        _ => p.expected("expression"),
    }

    if continuable && min_prec < 3 && p.prev_end() == p.current_start() && maybe_delimited(p) {
        p.wrap(m, TypstSyntaxKind::Math);
    }

    // Whether there were _any_ primes in the loop.
    let mut primed = false;

    while !p.eof() && !p.at(stop) {
        if p.directly_at(TypstSyntaxKind::Text) && p.current_text() == "!" {
            p.eat();
            p.wrap(m, TypstSyntaxKind::Math);
            continue;
        }

        let prime_marker = p.marker();
        if p.eat_if_direct(TypstSyntaxKind::Prime) {
            // Eat as many primes as possible.
            while p.eat_if_direct(TypstSyntaxKind::Prime) {}
            p.wrap(prime_marker, TypstSyntaxKind::MathPrimes);

            // Will not be continued, so need to wrap the prime as attachment.
            if p.at(stop) {
                p.wrap(m, TypstSyntaxKind::MathAttach);
            }

            primed = true;
            continue;
        }

        // Separate primes and superscripts to different attachments.
        if primed && p.current() == TypstSyntaxKind::Hat {
            p.wrap(m, TypstSyntaxKind::MathAttach);
        }

        let Some((kind, stop, assoc, mut prec)) = math_op(p.current()) else {
            // No attachments, so we need to wrap primes as attachment.
            if primed {
                p.wrap(m, TypstSyntaxKind::MathAttach);
            }

            break;
        };

        if primed && kind == TypstSyntaxKind::MathFrac {
            p.wrap(m, TypstSyntaxKind::MathAttach);
        }

        if prec < min_prec {
            break;
        }

        match assoc {
            ast::Assoc::Left => prec += 1,
            ast::Assoc::Right => {}
        }

        if kind == TypstSyntaxKind::MathFrac {
            math_unparen(p, m);
        }

        p.eat();
        let m2 = p.marker();
        math_expr_prec(p, prec, stop);
        math_unparen(p, m2);

        if p.eat_if(TypstSyntaxKind::Underscore) || (!primed && p.eat_if(TypstSyntaxKind::Hat)) {
            let m3 = p.marker();
            math_expr_prec(p, prec, TypstSyntaxKind::Eof);
            math_unparen(p, m3);
        }

        p.wrap(m, kind);
    }
}

fn maybe_delimited(p: &mut Parser) -> bool {
    let open = math_class(p.current_text()) == Some(MathClass::Opening);
    if open {
        math_delimited(p);
    }
    open
}

fn math_delimited(p: &mut Parser) {
    let m = p.marker();
    p.eat();
    let m2 = p.marker();
    while !p.eof() && !p.at(TypstSyntaxKind::Dollar) {
        if math_class(p.current_text()) == Some(MathClass::Closing) {
            p.wrap(m2, TypstSyntaxKind::Math);
            p.eat();
            p.wrap(m, TypstSyntaxKind::MathDelimited);
            return;
        }

        let prev = p.prev_end();
        math_expr(p);
        if !p.progress(prev) {
            p.unexpected();
        }
    }

    p.wrap(m, TypstSyntaxKind::Math);
}

fn math_unparen(p: &mut Parser, m: Marker) {
    let Some(node) = p.nodes.get_mut(m.0) else {
        return;
    };
    if node.kind() != TypstSyntaxKind::MathDelimited {
        return;
    }

    if let [first, .., last] = node.children_mut() {
        if first.text() == "(" && last.text() == ")" {
            first.convert_to_kind(TypstSyntaxKind::LeftParen);
            last.convert_to_kind(TypstSyntaxKind::RightParen);
        }
    }

    node.convert_to_kind(TypstSyntaxKind::Math);
}

fn math_class(text: &str) -> Option<MathClass> {
    match text {
        "[|" => return Some(MathClass::Opening),
        "|]" => return Some(MathClass::Closing),
        "||" => return Some(MathClass::Fence),
        _ => {}
    }

    let mut chars = text.chars();
    chars
        .next()
        .filter(|_| chars.next().is_none())
        .and_then(unicode_math_class::class)
}

fn math_op(kind: TypstSyntaxKind) -> Option<(TypstSyntaxKind, TypstSyntaxKind, ast::Assoc, usize)> {
    match kind {
        TypstSyntaxKind::Underscore => Some((
            TypstSyntaxKind::MathAttach,
            TypstSyntaxKind::Hat,
            ast::Assoc::Right,
            2,
        )),
        TypstSyntaxKind::Hat => Some((
            TypstSyntaxKind::MathAttach,
            TypstSyntaxKind::Underscore,
            ast::Assoc::Right,
            2,
        )),
        TypstSyntaxKind::Slash => Some((
            TypstSyntaxKind::MathFrac,
            TypstSyntaxKind::Eof,
            ast::Assoc::Left,
            1,
        )),
        _ => None,
    }
}

fn math_args(p: &mut Parser) {
    let m = p.marker();
    p.convert(TypstSyntaxKind::LeftParen);

    let mut namable = true;
    let mut named = None;
    let mut has_arrays = false;
    let mut array = p.marker();
    let mut arg = p.marker();

    while !p.eof() && !p.at(TypstSyntaxKind::Dollar) {
        if namable
            && (p.at(TypstSyntaxKind::MathIdent) || p.at(TypstSyntaxKind::Text))
            && p.text[p.current_end()..].starts_with(':')
        {
            p.convert(TypstSyntaxKind::Ident);
            p.convert(TypstSyntaxKind::Colon);
            named = Some(arg);
            arg = p.marker();
            array = p.marker();
        }

        match p.current_text() {
            ")" => break,
            ";" => {
                maybe_wrap_in_math(p, arg, named);
                p.wrap(array, TypstSyntaxKind::Array);
                p.convert(TypstSyntaxKind::Semicolon);
                array = p.marker();
                arg = p.marker();
                namable = true;
                named = None;
                has_arrays = true;
                continue;
            }
            "," => {
                maybe_wrap_in_math(p, arg, named);
                p.convert(TypstSyntaxKind::Comma);
                arg = p.marker();
                namable = true;
                if named.is_some() {
                    array = p.marker();
                    named = None;
                }
                continue;
            }
            _ => {}
        }

        let prev = p.prev_end();
        math_expr(p);
        if !p.progress(prev) {
            p.unexpected();
        }

        namable = false;
    }

    if arg != p.marker() {
        maybe_wrap_in_math(p, arg, named);
        if named.is_some() {
            array = p.marker();
        }
    }

    if has_arrays && array != p.marker() {
        p.wrap(array, TypstSyntaxKind::Array);
    }

    if p.at(TypstSyntaxKind::Text) && p.current_text() == ")" {
        p.convert(TypstSyntaxKind::RightParen);
    } else {
        p.expected("closing paren");
        p.balanced = false;
    }

    p.wrap(m, TypstSyntaxKind::Args);
}

fn maybe_wrap_in_math(p: &mut Parser, arg: Marker, named: Option<Marker>) {
    let exprs = p
        .post_process(arg)
        .filter(|node| node.is::<ast::Expr>())
        .count();
    if exprs != 1 {
        p.wrap(arg, TypstSyntaxKind::Math);
    }

    if let Some(m) = named {
        p.wrap(m, TypstSyntaxKind::Named);
    }
}

fn code(p: &mut Parser, stop: impl FnMut(&Parser) -> bool) {
    let m = p.marker();
    code_exprs(p, stop);
    p.wrap(m, TypstSyntaxKind::Code);
}

fn code_exprs(p: &mut Parser, mut stop: impl FnMut(&Parser) -> bool) {
    while !p.eof() && !stop(p) {
        p.enter_newline_mode(NewlineMode::Contextual);
        let prev = p.prev_end();
        code_expr(p);
        if p.progress(prev) && !p.eof() && !stop(p) && !p.eat_if(TypstSyntaxKind::Semicolon) {
            p.expected("semicolon or line break");
        }
        p.exit_newline_mode();
        if !p.progress(prev) && !p.eof() {
            p.unexpected();
        }
    }
}

fn code_expr(p: &mut Parser) {
    code_expr_prec(p, false, 0, false)
}

fn code_expr_or_pattern(p: &mut Parser) {
    code_expr_prec(p, false, 0, true)
}

fn embedded_code_expr(p: &mut Parser) {
    p.enter_newline_mode(NewlineMode::Stop);
    p.enter(LexMode::Code);
    p.assert(TypstSyntaxKind::Hash);
    p.unskip();

    let stmt = matches!(
        p.current(),
        TypstSyntaxKind::Let
            | TypstSyntaxKind::Set
            | TypstSyntaxKind::Show
            | TypstSyntaxKind::Import
            | TypstSyntaxKind::Include
            | TypstSyntaxKind::Return
    );

    let prev = p.prev_end();
    code_expr_prec(p, true, 0, false);

    // Consume error for things like `#12p` or `#"abc\"`.#
    if !p.progress(prev) && !p.current().is_trivia() && !p.eof() {
        p.unexpected();
    }

    let semi =
        (stmt || p.directly_at(TypstSyntaxKind::Semicolon)) && p.eat_if(TypstSyntaxKind::Semicolon);

    if stmt && !semi && !p.eof() && !p.at(TypstSyntaxKind::RightDelimiter) {
        p.expected("semicolon or line break");
    }

    p.exit();
    p.exit_newline_mode();
}

fn code_expr_prec(p: &mut Parser, atomic: bool, min_prec: usize, allow_destructuring: bool) {
    let m = p.marker();
    if let (false, Some(op)) = (atomic, ast::UnOp::from_kind(p.current())) {
        p.eat();
        code_expr_prec(p, atomic, op.precedence(), false);
        p.wrap(m, TypstSyntaxKind::Unary);
    } else {
        code_primary(p, atomic, allow_destructuring);
    }

    loop {
        if p.directly_at(TypstSyntaxKind::LeftParen)
            || p.directly_at(TypstSyntaxKind::LeftDelimiter)
        {
            args(p);
            p.wrap(m, TypstSyntaxKind::FuncCall);
            continue;
        }

        let at_field_or_method =
            p.directly_at(TypstSyntaxKind::Dot) && p.lexer.clone().next() == TypstSyntaxKind::Ident;

        if atomic && !at_field_or_method {
            break;
        }

        if p.eat_if(TypstSyntaxKind::Dot) {
            p.expect(TypstSyntaxKind::Ident);
            p.wrap(m, TypstSyntaxKind::FieldAccess);
            continue;
        }

        let binop = if ast::BinOp::NotIn.precedence() >= min_prec && p.eat_if(TypstSyntaxKind::Not)
        {
            if p.at(TypstSyntaxKind::In) {
                Some(ast::BinOp::NotIn)
            } else {
                p.expected("keyword `in`");
                break;
            }
        } else {
            ast::BinOp::from_kind(p.current())
        };

        if let Some(op) = binop {
            let mut prec = op.precedence();
            if prec < min_prec {
                break;
            }

            match op.assoc() {
                ast::Assoc::Left => prec += 1,
                ast::Assoc::Right => {}
            }

            p.eat();
            code_expr_prec(p, false, prec, false);
            p.wrap(m, TypstSyntaxKind::Binary);
            continue;
        }

        break;
    }
}

fn code_primary(p: &mut Parser, atomic: bool, allow_destructuring: bool) {
    let m = p.marker();
    match p.current() {
        TypstSyntaxKind::Ident => {
            p.eat();
            if !atomic && p.at(TypstSyntaxKind::Arrow) {
                p.wrap(m, TypstSyntaxKind::Params);
                p.assert(TypstSyntaxKind::Arrow);
                code_expr(p);
                p.wrap(m, TypstSyntaxKind::Closure);
            }
        }
        TypstSyntaxKind::Underscore if !atomic => {
            p.eat();
            if p.at(TypstSyntaxKind::Arrow) {
                p.wrap(m, TypstSyntaxKind::Params);
                p.eat();
                code_expr(p);
                p.wrap(m, TypstSyntaxKind::Closure);
            } else if let Some(underscore) = p.node_mut(m) {
                underscore.convert_to_error("expected expression, found underscore");
            }
        }

        TypstSyntaxKind::LeftBrace => code_block(p),
        TypstSyntaxKind::LeftDelimiter => content_block(p),
        TypstSyntaxKind::LeftParen => with_paren(p, allow_destructuring),
        TypstSyntaxKind::Dollar => equation(p),
        TypstSyntaxKind::Let => let_binding(p),
        TypstSyntaxKind::Set => set_rule(p),
        TypstSyntaxKind::Show => show_rule(p),
        TypstSyntaxKind::If => conditional(p),
        TypstSyntaxKind::While => while_loop(p),
        TypstSyntaxKind::For => for_loop(p),
        TypstSyntaxKind::Import => module_import(p),
        TypstSyntaxKind::Include => module_include(p),
        TypstSyntaxKind::Break => break_stmt(p),
        TypstSyntaxKind::Continue => continue_stmt(p),
        TypstSyntaxKind::Return => return_stmt(p),

        TypstSyntaxKind::None
        | TypstSyntaxKind::Auto
        | TypstSyntaxKind::Int
        | TypstSyntaxKind::Float
        | TypstSyntaxKind::Bool
        | TypstSyntaxKind::Numeric
        | TypstSyntaxKind::Str
        | TypstSyntaxKind::Label
        | TypstSyntaxKind::Raw => p.eat(),

        _ => p.expected("expression"),
    }
}

fn block(p: &mut Parser) {
    match p.current() {
        TypstSyntaxKind::LeftDelimiter => content_block(p),
        TypstSyntaxKind::LeftBrace => code_block(p),
        _ => p.expected("block"),
    }
}

pub(super) fn reparse_block(text: &str, range: Range<usize>) -> Option<TypstSyntaxNode> {
    let mut p = Parser::new(text, range.start, LexMode::Code);
    assert!(p.at(TypstSyntaxKind::LeftDelimiter) || p.at(TypstSyntaxKind::LeftBrace));
    block(&mut p);
    (p.balanced && p.prev_end() == range.end).then(|| p.finish().into_iter().next().unwrap())
}

fn code_block(p: &mut Parser) {
    let m = p.marker();
    p.enter(LexMode::Code);
    p.enter_newline_mode(NewlineMode::Continue);
    p.assert(TypstSyntaxKind::LeftBrace);
    code(p, |p| {
        p.at(TypstSyntaxKind::RightBrace)
            || p.at(TypstSyntaxKind::RightDelimiter)
            || p.at(TypstSyntaxKind::RightParen)
    });
    p.expect_closing_delimiter(m, TypstSyntaxKind::RightBrace);
    p.exit();
    p.exit_newline_mode();
    p.wrap(m, TypstSyntaxKind::CodeBlock);
}

fn content_block(p: &mut Parser) {
    let m = p.marker();
    p.enter(LexMode::TypstMarkup);
    p.assert(TypstSyntaxKind::LeftDelimiter);
    markup(p, true, 0, |p| p.at(TypstSyntaxKind::RightDelimiter));
    p.expect_closing_delimiter(m, TypstSyntaxKind::RightDelimiter);
    p.exit();
    p.wrap(m, TypstSyntaxKind::ContentBlock);
}

fn with_paren(p: &mut Parser, allow_destructuring: bool) {
    let m = p.marker();
    let mut kind = collection(p, true);
    if p.at(TypstSyntaxKind::Arrow) {
        validate_params_at(p, m);
        p.wrap(m, TypstSyntaxKind::Params);
        p.assert(TypstSyntaxKind::Arrow);
        code_expr(p);
        kind = TypstSyntaxKind::Closure;
    } else if p.at(TypstSyntaxKind::Eq) && kind != TypstSyntaxKind::Parenthesized {
        // TODO: add warning if p.at(TypstSyntaxKind::Eq) && kind == TypstSyntaxKind::Parenthesized

        validate_pattern_at(p, m, false);
        p.wrap(m, TypstSyntaxKind::Destructuring);
        p.assert(TypstSyntaxKind::Eq);
        code_expr(p);
        kind = TypstSyntaxKind::DestructAssignment;
    }

    match kind {
        TypstSyntaxKind::Array if !allow_destructuring => validate_array_at(p, m),
        TypstSyntaxKind::Dict if !allow_destructuring => validate_dict_at(p, m),
        TypstSyntaxKind::Parenthesized if !allow_destructuring => validate_parenthesized_at(p, m),
        TypstSyntaxKind::Destructuring if !allow_destructuring => invalidate_destructuring(p, m),
        _ => {}
    }
    p.wrap(m, kind);
}

fn invalidate_destructuring(p: &mut Parser, m: Marker) {
    let mut collection_kind = Option::None;
    for child in p.post_process(m) {
        match child.kind() {
            TypstSyntaxKind::Named | TypstSyntaxKind::Keyed => match collection_kind {
                Some(TypstSyntaxKind::Array) => child.convert_to_error(eco_format!(
                    "expected expression, found {}",
                    child.kind().name()
                )),
                _ => collection_kind = Some(TypstSyntaxKind::Dict),
            },
            TypstSyntaxKind::LeftParen | TypstSyntaxKind::RightParen | TypstSyntaxKind::Comma => {}
            kind => match collection_kind {
                Some(TypstSyntaxKind::Dict) => child.convert_to_error(eco_format!(
                    "expected named or keyed pair, found {}",
                    kind.name()
                )),
                _ => collection_kind = Some(TypstSyntaxKind::Array),
            },
        }
    }
}

fn collection(p: &mut Parser, keyed: bool) -> TypstSyntaxKind {
    p.enter_newline_mode(NewlineMode::Continue);

    let m = p.marker();
    p.assert(TypstSyntaxKind::LeftParen);

    let mut count = 0;
    let mut parenthesized = true;
    let mut kind = None;
    if keyed && p.eat_if(TypstSyntaxKind::Colon) {
        kind = Some(TypstSyntaxKind::Dict);
        parenthesized = false;
    }

    while !p.current().is_terminator() {
        let prev = p.prev_end();
        match item(p, keyed) {
            TypstSyntaxKind::Spread => parenthesized = false,
            TypstSyntaxKind::Named | TypstSyntaxKind::Keyed => {
                match kind {
                    Some(TypstSyntaxKind::Array) => kind = Some(TypstSyntaxKind::Destructuring),
                    _ => kind = Some(TypstSyntaxKind::Dict),
                }
                parenthesized = false;
            }
            TypstSyntaxKind::Int => match kind {
                Some(TypstSyntaxKind::Array) | None => kind = Some(TypstSyntaxKind::Array),
                Some(_) => kind = Some(TypstSyntaxKind::Destructuring),
            },
            _ if kind.is_none() => kind = Some(TypstSyntaxKind::Array),
            _ => {}
        }

        if !p.progress(prev) {
            p.unexpected();
            continue;
        }

        count += 1;

        if p.current().is_terminator() {
            break;
        }

        if p.expect(TypstSyntaxKind::Comma) {
            parenthesized = false;
        }
    }

    p.expect_closing_delimiter(m, TypstSyntaxKind::RightParen);
    p.exit_newline_mode();

    if parenthesized && count == 1 {
        TypstSyntaxKind::Parenthesized
    } else {
        kind.unwrap_or(TypstSyntaxKind::Array)
    }
}

fn item(p: &mut Parser, keyed: bool) -> TypstSyntaxKind {
    let m = p.marker();

    if p.eat_if(TypstSyntaxKind::Dots) {
        if p.at(TypstSyntaxKind::Comma) || p.at(TypstSyntaxKind::RightParen) {
            p.wrap(m, TypstSyntaxKind::Spread);
            return TypstSyntaxKind::Spread;
        }

        code_expr(p);
        p.wrap(m, TypstSyntaxKind::Spread);
        return TypstSyntaxKind::Spread;
    }

    if p.at(TypstSyntaxKind::Underscore) {
        // This is a temporary workaround to fix `v.map(_ => {})`.
        let mut lexer = p.lexer.clone();
        let next = std::iter::from_fn(|| Some(lexer.next())).find(|kind| !kind.is_trivia());
        if next != Some(TypstSyntaxKind::Arrow) {
            p.eat();
            return TypstSyntaxKind::Underscore;
        }
    }

    code_expr_or_pattern(p);

    if !p.eat_if(TypstSyntaxKind::Colon) {
        return TypstSyntaxKind::Int;
    }

    if !p.eat_if(TypstSyntaxKind::Underscore) {
        code_expr(p);
    }

    let kind = match p.node(m).map(TypstSyntaxNode::kind) {
        Some(TypstSyntaxKind::Ident) => TypstSyntaxKind::Named,
        Some(_) if keyed => TypstSyntaxKind::Keyed,
        _ => {
            for child in p.post_process(m) {
                if child.kind() == TypstSyntaxKind::Colon {
                    break;
                }

                let expected = if keyed { "expression" } else { "identifier" };
                let message = eco_format!(
                    "expected {expected}, found {found}",
                    found = child.kind().name(),
                );
                child.convert_to_error(message);
            }
            TypstSyntaxKind::Named
        }
    };

    p.wrap(m, kind);
    kind
}

fn args(p: &mut Parser) {
    if !p.at(TypstSyntaxKind::LeftParen) && !p.at(TypstSyntaxKind::LeftDelimiter) {
        p.expected("argument list");
    }

    let m = p.marker();
    if p.at(TypstSyntaxKind::LeftParen) {
        collection(p, false);
        validate_args_at(p, m);
    }

    while p.directly_at(TypstSyntaxKind::LeftDelimiter) {
        content_block(p);
    }

    p.wrap(m, TypstSyntaxKind::Args);
}

enum PatternKind {
    Ident,
    Other,
}

fn pattern(p: &mut Parser) -> PatternKind {
    let m = p.marker();
    if p.at(TypstSyntaxKind::LeftParen) {
        let kind = collection(p, false);
        validate_pattern_at(p, m, true);

        if kind != TypstSyntaxKind::Parenthesized {
            p.wrap(m, TypstSyntaxKind::Destructuring);
        }
        PatternKind::Other
    } else if p.eat_if(TypstSyntaxKind::Underscore) {
        PatternKind::Other
    } else {
        p.expect(TypstSyntaxKind::Ident);
        PatternKind::Ident
    }
}

fn let_binding(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Let);

    let m2 = p.marker();
    let mut closure = false;
    let mut other = false;
    match pattern(p) {
        PatternKind::Ident => {
            if p.directly_at(TypstSyntaxKind::LeftParen) {
                let m3 = p.marker();
                collection(p, false);
                validate_params_at(p, m3);
                p.wrap(m3, TypstSyntaxKind::Params);
                closure = true;
            }
        }
        PatternKind::Other => other = true,
    }

    let f = if closure || other {
        Parser::expect
    } else {
        Parser::eat_if
    };
    if f(p, TypstSyntaxKind::Eq) {
        code_expr(p);
    }

    if closure {
        p.wrap(m2, TypstSyntaxKind::Closure);
    }

    p.wrap(m, TypstSyntaxKind::LetBinding);
}

fn set_rule(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Set);

    let m2 = p.marker();
    p.expect(TypstSyntaxKind::Ident);
    while p.eat_if(TypstSyntaxKind::Dot) {
        p.expect(TypstSyntaxKind::Ident);
        p.wrap(m2, TypstSyntaxKind::FieldAccess);
    }

    args(p);
    if p.eat_if(TypstSyntaxKind::If) {
        code_expr(p);
    }
    p.wrap(m, TypstSyntaxKind::SetRule);
}

fn show_rule(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Show);
    let m2 = p.before_trivia();

    if !p.at(TypstSyntaxKind::Colon) {
        code_expr(p);
    }

    if p.eat_if(TypstSyntaxKind::Colon) {
        code_expr(p);
    } else {
        p.expected_at(m2, "colon");
    }

    p.wrap(m, TypstSyntaxKind::ShowRule);
}

fn conditional(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::If);
    code_expr(p);
    block(p);
    if p.eat_if(TypstSyntaxKind::Else) {
        if p.at(TypstSyntaxKind::If) {
            conditional(p);
        } else {
            block(p);
        }
    }
    p.wrap(m, TypstSyntaxKind::Conditional);
}

fn while_loop(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::While);
    code_expr(p);
    block(p);
    p.wrap(m, TypstSyntaxKind::WhileLoop);
}

fn for_loop(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::For);
    pattern(p);
    if p.at(TypstSyntaxKind::Comma) {
        p.expected("keyword `in`");
        p.hint("did you mean to use a destructuring pattern?");
        if !p.eat_if(TypstSyntaxKind::Ident) {
            p.eat_if(TypstSyntaxKind::Underscore);
        }
        p.eat_if(TypstSyntaxKind::In);
    } else {
        p.expect(TypstSyntaxKind::In);
    }
    code_expr(p);
    block(p);
    p.wrap(m, TypstSyntaxKind::ForLoop);
}

fn module_import(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Import);
    code_expr(p);
    if p.eat_if(TypstSyntaxKind::As) {
        // Allow renaming a full module import.
        // If items are included, both the full module and the items are
        // imported at the same time.
        p.expect(TypstSyntaxKind::Ident);
    }
    if p.eat_if(TypstSyntaxKind::Colon) && !p.eat_if(TypstSyntaxKind::Star) {
        import_items(p);
    }
    p.wrap(m, TypstSyntaxKind::ModuleImport);
}

fn import_items(p: &mut Parser) {
    let m = p.marker();
    while !p.eof() && !p.at(TypstSyntaxKind::Semicolon) {
        let item_marker = p.marker();
        if !p.eat_if(TypstSyntaxKind::Ident) {
            p.unexpected();
        }

        // Rename imported item.
        if p.eat_if(TypstSyntaxKind::As) {
            p.expect(TypstSyntaxKind::Ident);
            p.wrap(item_marker, TypstSyntaxKind::RenamedImportItem);
        }

        if p.current().is_terminator() {
            break;
        }
        p.expect(TypstSyntaxKind::Comma);
    }
    p.wrap(m, TypstSyntaxKind::ImportItems);
}

fn module_include(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Include);
    code_expr(p);
    p.wrap(m, TypstSyntaxKind::ModuleInclude);
}

fn break_stmt(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Break);
    p.wrap(m, TypstSyntaxKind::LoopBreak);
}

fn continue_stmt(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Continue);
    p.wrap(m, TypstSyntaxKind::LoopContinue);
}

fn return_stmt(p: &mut Parser) {
    let m = p.marker();
    p.assert(TypstSyntaxKind::Return);
    if !p.current().is_terminator() && !p.at(TypstSyntaxKind::Comma) {
        code_expr(p);
    }
    p.wrap(m, TypstSyntaxKind::FuncReturn);
}

fn validate_parenthesized_at(p: &mut Parser, m: Marker) {
    for child in p.post_process(m) {
        let kind = child.kind();
        match kind {
            TypstSyntaxKind::Array => validate_array(child.children_mut().iter_mut()),
            TypstSyntaxKind::Dict => validate_dict(child.children_mut().iter_mut()),
            TypstSyntaxKind::Underscore => {
                child.convert_to_error(eco_format!("expected expression, found {}", kind.name()));
            }
            _ => {}
        }
    }
}

fn validate_array_at(p: &mut Parser, m: Marker) {
    validate_array(p.post_process(m))
}

fn validate_array<'a>(children: impl Iterator<Item = &'a mut TypstSyntaxNode>) {
    for child in children {
        let kind = child.kind();
        match kind {
            TypstSyntaxKind::Array => validate_array(child.children_mut().iter_mut()),
            TypstSyntaxKind::Dict => validate_dict(child.children_mut().iter_mut()),
            TypstSyntaxKind::Named | TypstSyntaxKind::Keyed | TypstSyntaxKind::Underscore => {
                child.convert_to_error(eco_format!("expected expression, found {}", kind.name()));
            }
            _ => {}
        }
    }
}

fn validate_dict_at(p: &mut Parser, m: Marker) {
    validate_dict(p.post_process(m))
}

fn validate_dict<'a>(children: impl Iterator<Item = &'a mut TypstSyntaxNode>) {
    let mut used = HashSet::new();
    for child in children {
        match child.kind() {
            TypstSyntaxKind::Named | TypstSyntaxKind::Keyed => {
                let Some(first) = child.children_mut().first_mut() else {
                    continue;
                };
                let key = if let Some(str) = first.cast::<ast::Str>() {
                    str.get()
                } else if let Some(ident) = first.cast::<ast::Ident>() {
                    ident.get().clone()
                } else {
                    continue;
                };

                if !used.insert(key.clone()) {
                    first.convert_to_error(eco_format!("duplicate key: {key}"));
                    child.make_erroneous();
                }
            }
            TypstSyntaxKind::Spread => {}
            TypstSyntaxKind::LeftParen
            | TypstSyntaxKind::RightParen
            | TypstSyntaxKind::Comma
            | TypstSyntaxKind::Colon
            | TypstSyntaxKind::Space => {}
            kind => {
                child.convert_to_error(eco_format!(
                    "expected named or keyed pair, found {}",
                    kind.name()
                ));
            }
        }
    }
}

fn validate_params_at(p: &mut Parser, m: Marker) {
    let mut used_spread = false;
    let mut used = HashSet::new();
    for child in p.post_process(m) {
        match child.kind() {
            TypstSyntaxKind::Ident => {
                if !used.insert(child.text().clone()) {
                    child.convert_to_error(eco_format!("duplicate parameter: {}", child.text()));
                }
            }
            TypstSyntaxKind::Named => {
                let Some(within) = child.children_mut().first_mut() else {
                    return;
                };
                if !used.insert(within.text().clone()) {
                    within.convert_to_error(eco_format!("duplicate parameter: {}", within.text()));
                    child.make_erroneous();
                }
            }
            TypstSyntaxKind::Spread => {
                let Some(within) = child.children_mut().last_mut() else {
                    continue;
                };
                if used_spread {
                    child.convert_to_error("only one argument sink is allowed");
                    continue;
                }
                used_spread = true;
                if within.kind() == TypstSyntaxKind::Dots {
                    continue;
                } else if within.kind() != TypstSyntaxKind::Ident {
                    within.convert_to_error(eco_format!(
                        "expected identifier, found {}",
                        within.kind().name(),
                    ));
                    child.make_erroneous();
                    continue;
                }
                if !used.insert(within.text().clone()) {
                    within.convert_to_error(eco_format!("duplicate parameter: {}", within.text()));
                    child.make_erroneous();
                }
            }
            TypstSyntaxKind::Array | TypstSyntaxKind::Dict | TypstSyntaxKind::Destructuring => {
                validate_pattern(child.children_mut().iter_mut(), &mut used, false);
                child.convert_to_kind(TypstSyntaxKind::Destructuring);
            }
            TypstSyntaxKind::LeftParen
            | TypstSyntaxKind::RightParen
            | TypstSyntaxKind::Comma
            | TypstSyntaxKind::Underscore => {}
            kind => {
                child.convert_to_error(eco_format!(
                    "expected identifier, named pair or argument sink, found {}",
                    kind.name()
                ));
            }
        }
    }
}

fn validate_args_at(p: &mut Parser, m: Marker) {
    let mut used = HashSet::new();
    for child in p.post_process(m) {
        if child.kind() == TypstSyntaxKind::Named {
            let Some(within) = child.children_mut().first_mut() else {
                return;
            };
            if !used.insert(within.text().clone()) {
                within.convert_to_error(eco_format!("duplicate argument: {}", within.text()));
                child.make_erroneous();
            }
        } else if child.kind() == TypstSyntaxKind::Underscore {
            child.convert_to_error("unexpected underscore");
        }
    }
}

fn validate_pattern_at(p: &mut Parser, m: Marker, forbid_expressions: bool) {
    let mut used = HashSet::new();
    validate_pattern(p.post_process(m), &mut used, forbid_expressions);
}

fn validate_pattern<'a>(
    children: impl Iterator<Item = &'a mut TypstSyntaxNode>,
    used: &mut HashSet<EcoString>,
    forbid_expressions: bool,
) {
    let mut used_spread = false;
    for child in children {
        match child.kind() {
            TypstSyntaxKind::Ident => {
                if !used.insert(child.text().clone()) {
                    child.convert_to_error("at most one binding per identifier is allowed");
                }
            }
            TypstSyntaxKind::Spread => {
                let Some(within) = child.children_mut().last_mut() else {
                    continue;
                };
                if used_spread {
                    child.convert_to_error("at most one destructuring sink is allowed");
                    continue;
                }
                used_spread = true;

                if within.kind() == TypstSyntaxKind::Dots {
                    continue;
                } else if forbid_expressions && within.kind() != TypstSyntaxKind::Ident {
                    within.convert_to_error(eco_format!(
                        "expected identifier, found {}",
                        within.kind().name(),
                    ));
                    child.make_erroneous();
                    continue;
                }

                if !used.insert(within.text().clone()) {
                    within.convert_to_error("at most one binding per identifier is allowed");
                    child.make_erroneous();
                }
            }
            TypstSyntaxKind::Named => {
                let Some(within) = child.children_mut().first_mut() else {
                    return;
                };
                if !used.insert(within.text().clone()) {
                    within.convert_to_error("at most one binding per identifier is allowed");
                    child.make_erroneous();
                }

                if forbid_expressions {
                    let Some(within) = child.children_mut().last_mut() else {
                        return;
                    };
                    if within.kind() != TypstSyntaxKind::Ident
                        && within.kind() != TypstSyntaxKind::Underscore
                    {
                        within.convert_to_error(eco_format!(
                            "expected identifier, found {}",
                            within.kind().name(),
                        ));
                        child.make_erroneous();
                    }
                }
            }
            TypstSyntaxKind::LeftParen
            | TypstSyntaxKind::RightParen
            | TypstSyntaxKind::Comma
            | TypstSyntaxKind::Underscore => {}
            kind => {
                if forbid_expressions {
                    child.convert_to_error(eco_format!(
                        "expected identifier or destructuring sink, found {}",
                        kind.name()
                    ));
                }
            }
        }
    }
}

/// Manages parsing of a stream of tokens.
struct Parser<'s> {
    text: &'s str,
    lexer: Lexer<'s>,
    prev_end: usize,
    current_start: usize,
    current: TypstSyntaxKind,
    modes: Vec<LexMode>,
    nodes: Vec<TypstSyntaxNode>,
    newline_modes: Vec<NewlineMode>,
    balanced: bool,
}

/// How to proceed with parsing when seeing a newline.
enum NewlineMode {
    /// Stop always.
    Stop,
    /// Proceed if there is no continuation with `else` or `.`
    Contextual,
    /// Just proceed like with normal whitespace.
    Continue,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Marker(usize);

impl<'s> Parser<'s> {
    fn new(text: &'s str, offset: usize, mode: LexMode) -> Self {
        let mut lexer = Lexer::new(text, mode);
        lexer.jump(offset);
        let current = lexer.next();
        Self {
            lexer,
            text,
            prev_end: offset,
            current_start: offset,
            current,
            modes: vec![],
            nodes: vec![],
            newline_modes: vec![],
            balanced: true,
        }
    }

    fn finish(self) -> Vec<TypstSyntaxNode> {
        self.nodes
    }

    fn prev_end(&self) -> usize {
        self.prev_end
    }

    fn current(&self) -> TypstSyntaxKind {
        self.current
    }

    fn current_start(&self) -> usize {
        self.current_start
    }

    fn current_end(&self) -> usize {
        self.lexer.cursor()
    }

    fn current_text(&self) -> &'s str {
        &self.text[self.current_start..self.current_end()]
    }

    fn at(&self, kind: TypstSyntaxKind) -> bool {
        self.current == kind
    }

    #[track_caller]
    fn assert(&mut self, kind: TypstSyntaxKind) {
        assert_eq!(self.current, kind);
        self.eat();
    }

    fn eof(&self) -> bool {
        self.at(TypstSyntaxKind::Eof)
    }

    fn directly_at(&self, kind: TypstSyntaxKind) -> bool {
        self.current == kind && self.prev_end == self.current_start
    }

    /// Eats if at `kind`.
    ///
    /// Note: In math and code mode, this will ignore trivia in front of the
    /// `kind`, To forbid skipping trivia, consider using `eat_if_direct`.
    fn eat_if(&mut self, kind: TypstSyntaxKind) -> bool {
        let at = self.at(kind);
        if at {
            self.eat();
        }
        at
    }

    /// Eats only if currently at the start of `kind`.
    fn eat_if_direct(&mut self, kind: TypstSyntaxKind) -> bool {
        let at = self.directly_at(kind);
        if at {
            self.eat();
        }
        at
    }

    fn convert(&mut self, kind: TypstSyntaxKind) {
        self.current = kind;
        self.eat();
    }

    fn newline(&mut self) -> bool {
        self.lexer.newline()
    }

    fn column(&self, at: usize) -> usize {
        self.text[..at]
            .chars()
            .rev()
            .take_while(|&c| !is_newline(c))
            .count()
    }

    fn marker(&self) -> Marker {
        Marker(self.nodes.len())
    }

    fn node(&self, m: Marker) -> Option<&TypstSyntaxNode> {
        self.nodes.get(m.0)
    }

    fn node_mut(&mut self, m: Marker) -> Option<&mut TypstSyntaxNode> {
        self.nodes.get_mut(m.0)
    }

    fn post_process(&mut self, m: Marker) -> impl Iterator<Item = &mut TypstSyntaxNode> {
        self.nodes[m.0..]
            .iter_mut()
            .filter(|child| !child.kind().is_error() && !child.kind().is_trivia())
    }

    fn wrap(&mut self, from: Marker, kind: TypstSyntaxKind) {
        self.wrap_within(from, self.before_trivia(), kind);
    }

    fn wrap_all(&mut self, from: Marker, kind: TypstSyntaxKind) {
        self.wrap_within(from, Marker(self.nodes.len()), kind)
    }

    fn wrap_within(&mut self, from: Marker, to: Marker, kind: TypstSyntaxKind) {
        let len = self.nodes.len();
        let to = to.0.min(len);
        let from = from.0.min(to);
        let children = self.nodes.drain(from..to).collect();
        self.nodes
            .insert(from, TypstSyntaxNode::inner(kind, children));
    }

    fn progress(&self, offset: usize) -> bool {
        offset < self.prev_end
    }

    fn enter(&mut self, mode: LexMode) {
        self.modes.push(self.lexer.mode());
        self.lexer.set_mode(mode);
    }

    fn exit(&mut self) {
        let mode = self.modes.pop().unwrap();
        if mode != self.lexer.mode() {
            self.unskip();
            self.lexer.set_mode(mode);
            self.lexer.jump(self.current_start);
            self.lex();
            self.skip();
        }
    }

    fn enter_newline_mode(&mut self, stop: NewlineMode) {
        self.newline_modes.push(stop);
    }

    fn exit_newline_mode(&mut self) {
        self.unskip();
        self.newline_modes.pop();
        self.lexer.jump(self.prev_end);
        self.lex();
        self.skip();
    }

    fn eat(&mut self) {
        self.save();
        self.lex();
        self.skip();
    }

    fn skip(&mut self) {
        if self.lexer.mode() != LexMode::TypstMarkup {
            while self.current.is_trivia() {
                self.save();
                self.lex();
            }
        }
    }

    fn unskip(&mut self) {
        if self.lexer.mode() != LexMode::TypstMarkup && self.prev_end != self.current_start {
            while self
                .nodes
                .last()
                .map_or(false, |last| last.kind().is_trivia())
            {
                self.nodes.pop();
            }

            self.lexer.jump(self.prev_end);
            self.lex();
        }
    }

    fn save(&mut self) {
        let text = self.current_text();
        if self.at(TypstSyntaxKind::Error) {
            let message = self.lexer.take_error().unwrap();
            self.nodes.push(TypstSyntaxNode::error(message, text));
        } else {
            self.nodes.push(TypstSyntaxNode::leaf(self.current, text));
        }

        if self.lexer.mode() == LexMode::TypstMarkup || !self.current.is_trivia() {
            self.prev_end = self.current_end();
        }
    }

    fn lex(&mut self) {
        self.current_start = self.lexer.cursor();
        self.current = self.lexer.next();
        if self.lexer.mode() == LexMode::Code
            && self.lexer.newline()
            && match self.newline_modes.last() {
                Some(NewlineMode::Continue) => false,
                Some(NewlineMode::Contextual) => !matches!(
                    self.lexer.clone().next(),
                    TypstSyntaxKind::Else | TypstSyntaxKind::Dot
                ),
                Some(NewlineMode::Stop) => true,
                None => false,
            }
        {
            self.current = TypstSyntaxKind::Eof;
        }
    }
}

impl<'s> Parser<'s> {
    /// Consume the given syntax `kind` or produce an error.
    fn expect(&mut self, kind: TypstSyntaxKind) -> bool {
        let at = self.at(kind);
        if at {
            self.eat();
        } else if kind == TypstSyntaxKind::Ident && self.current.is_keyword() {
            let found_text = self.current_text();
            let found = self.current.name();
            self.expected_found(kind.name(), found);
            self.hint(eco_format!(
                "{} is not allowed as an identifier; try `{}_` instead",
                found,
                found_text
            ));
        } else {
            self.balanced &= !kind.is_grouping();
            self.expected(kind.name());
        }
        at
    }

    /// Produce an error that the given `thing` was expected.
    fn expected(&mut self, thing: &str) {
        if !self.after_error() {
            self.expected_at(self.before_trivia(), thing);
        }
    }

    /// Produce an error that the given `thing` was expected but another
    /// thing was `found` and consume the next token.
    fn expected_found(&mut self, thing: &str, found: &str) {
        self.trim_errors();
        self.convert_to_error(eco_format!("expected {thing}, found {found}"));
    }

    /// Produce an error that the given `thing` was expected at the position
    /// of the marker `m`.
    fn expected_at(&mut self, m: Marker, thing: &str) {
        let message = eco_format!("expected {}", thing);
        let error = TypstSyntaxNode::error(message, "");
        self.nodes.insert(m.0, error);
    }

    /// Produce an error for the unclosed delimiter `kind` at the position
    /// `open`.
    fn expect_closing_delimiter(&mut self, open: Marker, kind: TypstSyntaxKind) {
        if !self.eat_if(kind) {
            self.nodes[open.0].convert_to_error("unclosed delimiter");
        }
    }

    /// Consume the next token (if any) and produce an error stating that it was
    /// unexpected.
    fn unexpected(&mut self) {
        self.trim_errors();
        self.convert_to_error(eco_format!("unexpected {}", self.current.name()));
    }

    /// Consume the next token and turn it into an error.
    fn convert_to_error(&mut self, message: EcoString) {
        let kind = self.current;
        let offset = self.nodes.len();
        self.eat();
        self.balanced &= !kind.is_grouping();
        if !kind.is_error() {
            self.nodes[offset].convert_to_error(message);
        }
    }

    /// Adds a hint to the last node, if the last node is an error.
    fn hint(&mut self, hint: impl Into<EcoString>) {
        let m = self.before_trivia();
        if m.0 > 0 {
            self.nodes[m.0 - 1].hint(hint);
        }
    }

    /// Get a marker after the last non-trivia node.
    fn before_trivia(&self) -> Marker {
        let mut i = self.nodes.len();
        if self.lexer.mode() != LexMode::TypstMarkup && self.prev_end != self.current_start {
            while i > 0 && self.nodes[i - 1].kind().is_trivia() {
                i -= 1;
            }
        }
        Marker(i)
    }

    /// Whether the last non-trivia node is an error.
    fn after_error(&mut self) -> bool {
        let m = self.before_trivia();
        m.0 > 0 && self.nodes[m.0 - 1].kind().is_error()
    }

    /// Remove trailing errors with zero length.
    fn trim_errors(&mut self) {
        let Marker(end) = self.before_trivia();
        let mut start = end;
        while start > 0
            && self.nodes[start - 1].kind().is_error()
            && self.nodes[start - 1].is_empty()
        {
            start -= 1;
        }
        self.nodes.drain(start..end);
    }
}
