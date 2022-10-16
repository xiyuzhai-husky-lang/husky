use thin_vec::ThinVec;

use super::*;

#[macro_export]
macro_rules! deprecated_try_get {
    ($parser: expr, $patt:ident) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt() {
            Some(pattern)
        } else {
            $parser.rollback(saved_state);
            None
        }
    }};

    ($parser:expr, $patt:ident?) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt()? {
            Some(pattern)
        } else {
            $parser.rollback(saved_state);
            None
        }
    }};

    ($parser:expr, $patt:ident, $($args:expr),*) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt($($args),*) {
            Some(pattern)
        } else {
            $parser.rollback(saved_state);
            None
        }
    }};

    ($parser:expr, $patt:ident?, $($args:expr),*) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt($($args),*)? {
            Some(pattern)
        } else {
            $parser.rollback(saved_state);
            None
        }
    }};

    ($parser:expr, "_") => {{
        try_get!($parser, elide)
    }};
}

#[macro_export]
macro_rules! deprecated_try_eat {
    ($parser:expr, $patt:ident) => {{
        let saved_state = $parser.save_state();
        if $parser.$patt().is_some() {
            true
        } else {
            $parser.rollback(saved_state);
            false
        }
    }};

    ($parser:expr, $patt:ident, $($args:expr),*) => {{
        let saved_state = $parser.save_state();
        if $parser.$patt($($args),*).is_some() {
            true
        } else {
            $parser.rollback(saved_state);
            false
        }
    }};

    ($parser:expr, $patt:ident?, $($args:expr),*) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt($($args),*)? {
            true
        } else {
            $parser.rollback(saved_state);
            false
        }
    }};

    ($parser:expr, $patt:ident?, $($args:expr),*) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt($($args),*)? {
            true
        } else {
            $parser.rollback(saved_state);
            false
        }
    }};

    ($parser:expr, "->") => {{
        deprecated_try_eat!($parser, SpecialToken::BinaryOpr(BinaryOpr::Curry))
    }};

    ($parser:expr, "(") => {{
        deprecated_try_eat!($parser, SpecialToken::Bra(Bracket::Par))
    }};

    ($parser:expr, "<") => {{
        deprecated_try_eat!($parser, SpecialToken::LAngle)
    }};

    ($parser:expr, ":") => {{
        deprecated_try_eat!($parser, SpecialToken::Colon)
    }};

    ($parser:expr, "+") => {{
        deprecated_try_eat!($parser, SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Add)))
    }};

    ($parser:expr, "'") => {{
        deprecated_try_eat!($parser, SpecialToken::Lifetime)
    }};

    ($parser:expr, "&") => {{
        deprecated_try_eat!($parser, SpecialToken::Ambersand)
    }};

    ($parser:expr, "mut") => {{
        deprecated_try_eat!($parser, token_kind, TokenKind::Keyword(husky_word::Keyword::Liason(husky_word::LiasonKeyword::Mut)))
    }};

    ($parser:expr, "!!") => {{
        deprecated_try_eat!($parser, SpecialToken::DoubleExclamation)
    }};

    ($parser:expr, "_") => {{
        deprecated_try_eat!($parser, elide)
    }};

    ($parser:expr, $special:expr) => {{
        let saved_state = $parser.save_state();
        if $parser.special($special).is_some() {
            true
        } else {
            $parser.rollback(saved_state);
            false
        }
    }};
}

#[macro_export]
macro_rules! deprecated_get{
    ($parser: expr, $patt:ident) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt() {
            pattern
        } else {
            $parser.rollback(saved_state);
            return err!(
                format!("expect {}", stringify!($patt)),
                $parser.token_stream.next_range()
            )
        }
    }};

    ($parser:expr, $patt:ident, $($args:expr),*) => {{
        if let Some(pattern) = $parser.$patt($($args),*) {
            pattern
        } else {
            return err!(format!("expect `{}` after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, $patt:ident?) => {{
        let saved_state = $parser.save_state();
        if let Some(pattern) = $parser.$patt()? {
            pattern
        } else {
            $parser.rollback(saved_state);
            return err!(
                    format!("expect {}", stringify!($patt)),
                    $parser.token_stream.next_range()
                )
        }
    }};

    ($parser:expr, $patt:ident?, $($args:expr),*) => {{
        if let Some(pattern) = $parser.$patt($($args),*)? {
            pattern
        } else {
            return err!(format!("expect `{}` after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};


    ($parser:ident.$patt:ident?) => {{
        if let Some(pattern) = $parser.$patt()? {
            pattern
        } else {
            return err!(
                format!("expect `{}` after it", stringify!($patt)),
                $parser.token_stream.next_range()
            )
        }
    }};

    ($parser:ident.$patt:ident?($($args:expr),*)) => {{
        if let Some(pattern) = this.$patt($args,*)? {
            pattern
        } else {
            return err!(format!("expect `{}` after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};
}

#[macro_export]
macro_rules! get_patt {
    ($parser: expr, $patt: expr) => {{
        $parser.try_get(&$patt)?.ok_or(error!(
            format!("expect `{}` after it", $patt),
            $parser.token_stream.next_range()
        ))?
    }};
}

#[macro_export]
macro_rules! try_eat_special {
    ($parser: expr, $s: tt) => {{
        $parser.try_eat(&be_special_token_patt!($s))?
    }};
}

#[macro_export]
macro_rules! eat_special {
    ($parser: expr, $s: tt) => {{
        eat_patt!($parser, be_special_token_patt!($s))
    }};
}

#[macro_export]
macro_rules! eat_patt {
    ($parser: expr, $patt: expr) => {{
        if !$parser.try_eat(&$patt)? {
            return err!(
                format!(
                    "expect `{}` but get {} instead",
                    $patt,
                    if let Some(token) = $parser.token_stream.peek() {
                        format!("{token}")
                    } else {
                        "nothing".to_string()
                    }
                ),
                $parser.token_stream.next_range()
            );
        }
    }};
}

#[macro_export]
macro_rules! deprecated_eat {
    ($parser:expr, $patt:ident) => {{
        if $parser.$patt().is_none() {
            return err!(format!("expect `{}` after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, $patt:ident, $($args:expr),*) => {{
        if  $parser.$patt($($args),*).is_none() {
            return err!(
                format!("expect `{}` after it", stringify!($patt)),
                $parser.token_stream.next_range()
            )
        }
    }};

    ($parser:expr, $patt:ident?) => {{
        if $parser.$patt()?.is_none() {
            return err!(format!("expect `{}` after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, $patt:ident?, $($args:expr),*) => {{
        if  $parser.$patt($($args),*)?.is_none() {
            return err!(format!("expect `{}` after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, "(") => {{
        deprecated_eat!($parser, special, SpecialToken::Bra(Bracket::Par))
    }};

    ($parser:expr, "{") => {{
        deprecated_eat!($parser, special, SpecialToken::Bra(Bracket::Curl))
    }};

    ($parser:expr, ":") => {{
        deprecated_eat!($parser, special, SpecialToken::Colon)
    }};

    ($parser:expr, "=") => {{
        deprecated_eat!($parser, special, SpecialToken::BinaryOpr(BinaryOpr::Assign(None)))
    }};
}

#[macro_export]
macro_rules! comma_list {
    ($parser:expr, $first_patt:ident?, $second_patt:ident!, $terminator:ident) => {{
        let mut firsts = vec![];
        let mut seconds = vec![];
        let mut done = false;
        while let Some(item) = try_get!($parser, $first_patt?) {
            firsts.push(item);
            if !deprecated_try_eat!($parser, SpecialToken::Comma) {
                deprecated_eat!($parser, special, SpecialToken::$terminator);
                done = true;
                break;
            }
        }
        if !done && !deprecated_try_eat!($parser, SpecialToken::$terminator) {
            seconds.push($parser.$second_patt()?);
            loop {
                if deprecated_try_eat!($parser, SpecialToken::Comma) {
                    if deprecated_try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    seconds.push($parser.$second_patt()?);
                } else {
                    deprecated_eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        (firsts, seconds)
    }};

    ($parser:expr, $patt:ident, $terminator:ident) => {{
        let mut args = Defaut::default();
        if !deprecated_try_eat!($parser, SpecialToken::$terminator) {
            args.push(get!($parser, $patt?));
            loop {
                if deprecated_try_eat!($parser, SpecialToken::Comma) {
                    if deprecated_try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    args.push(get!($parser, $patt?));
                } else {
                    deprecated_eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!, $terminator:ident) => {{
        let mut args = vec![];
        if !deprecated_try_eat!($parser, SpecialToken::$terminator) {
            args.push($parser.$patt()?);
            loop {
                if deprecated_try_eat!($parser, SpecialToken::Comma) {
                    if deprecated_try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    args.push($parser.$patt()?);
                } else {
                    deprecated_eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!+, $terminator:ident) => {{
        let mut items = vec![$parser.$patt()?];
        loop {
            if !deprecated_try_eat!($parser, SpecialToken::Comma) {
                deprecated_eat!($parser, special, SpecialToken::$terminator);
                break;
            }
            if deprecated_try_eat!($parser, SpecialToken::$terminator) {
                break;
            }
            items.push($parser.$patt()?);
        }
        items
    }};

    ($parser:expr, $patt:ident!, ")") => {{
        comma_list!($parser, $patt!, RPar)
    }};

    ($parser:expr, $patt:ident!, "|") => {{
        comma_list!($parser, $patt!, Vertical)
    }};

    ($parser:expr, $patt:ident!+, ">") => {{
        comma_list!($parser, $patt!+, RAngle)
    }};
}

pub struct CommaListPattern<Item, Terminator>
where
    Item: AtomParserPattern,
    Terminator: AtomParserPattern,
{
    pub item: Item,
    pub terminator: Terminator,
}
impl<Item, Terminator> std::fmt::Display for CommaListPattern<Item, Terminator>
where
    Item: AtomParserPattern,
    Terminator: AtomParserPattern,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "comma list of {} till {}", self.item, self.terminator)
    }
}
impl<Item, Terminator> AtomParserPattern for CommaListPattern<Item, Terminator>
where
    Item: AtomParserPattern,
    Terminator: AtomParserPattern,
{
    type Output = Vec<Item::Output>;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        let mut args = vec![];
        if !parser.try_eat(&self.terminator)? {
            args.push(get_patt!(parser, self.item));
            loop {
                if try_eat_special!(parser, ",") {
                    if parser.try_eat(&self.terminator)? {
                        break;
                    }
                    args.push(get_patt!(parser, self.item));
                } else {
                    eat_patt!(parser, self.terminator);
                    break;
                }
            }
        }
        Ok(Some(args))
    }
}

pub struct ThinCommaListPattern<Item, Terminator>
where
    Item: AtomParserPattern,
    Terminator: AtomParserPattern,
{
    pub item: Item,
    pub terminator: Terminator,
}
impl<Item, Terminator> std::fmt::Display for ThinCommaListPattern<Item, Terminator>
where
    Item: AtomParserPattern,
    Terminator: AtomParserPattern,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl<Item, Terminator> AtomParserPattern for ThinCommaListPattern<Item, Terminator>
where
    Item: AtomParserPattern,
    Terminator: AtomParserPattern,
{
    type Output = ThinVec<Item::Output>;

    fn get_parsed(&self, parser: &mut AtomParser) -> AtomResult<Option<Self::Output>> {
        let mut args = thin_vec::thin_vec![];
        if !parser.try_eat(&self.terminator)? {
            if let Some(item) = parser.try_get(&self.item)? {
                args.push(item);
            } else {
                return Ok(None);
            }
            loop {
                if try_eat_special!(parser, ",") {
                    if parser.try_eat(&self.terminator)? {
                        break;
                    }
                    args.push(get_patt!(parser, self.item));
                } else {
                    eat_patt!(parser, self.terminator);
                    break;
                }
            }
        }
        Ok(Some(args))
    }
}

#[macro_export]
macro_rules! deprecated_thin_comma_list {
    ($parser:expr, $patt:ident!, ")") => {{
        deprecated_thin_comma_list!($parser, $patt!, SpecialToken::Ket(Bracket::Par))
    }};

    ($parser:expr, $first_patt:ident?, $second_patt:ident!, $terminator:expr) => {{
        let mut firsts = thin_vec![];
        let mut seconds = thin_vec![];
        let mut done = false;
        while let Some(item) = try_get!($parser, $first_patt?) {
            firsts.push(item);
            if !deprecated_try_eat!($parser, SpecialToken::Comma) {
                deprecated_eat!($parser, special, $terminator);
                done = true;
                break;
            }
        }
        if !done && !deprecated_try_eat!($parser, $terminator) {
            seconds.push($parser.$second_patt()?);
            loop {
                if deprecated_try_eat!($parser, SpecialToken::Comma) {
                    if deprecated_try_eat!($parser, $terminator) {
                        break;
                    }
                    seconds.push($parser.$second_patt()?);
                } else {
                    deprecated_eat!($parser, special, $terminator);
                    break;
                }
            }
        }
        (firsts, seconds)
    }};

    ($parser:expr, $patt:ident, $terminator:expr) => {{
        let mut args = thin_vec![];
        if !deprecated_try_eat!($parser, $terminator) {
            args.push(get!($parser, $patt?));
            loop {
                if deprecated_try_eat!($parser, SpecialToken::Comma) {
                    if deprecated_try_eat!($parser, $terminator) {
                        break;
                    }
                    args.push(get!($parser, $patt?));
                } else {
                    deprecated_eat!($parser, special, $terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!, $terminator:expr) => {{
        let mut args = thin_vec![];
        if !deprecated_try_eat!($parser, $terminator) {
            args.push($parser.$patt()?);
            loop {
                if deprecated_try_eat!($parser, SpecialToken::Comma) {
                    if deprecated_try_eat!($parser, $terminator) {
                        break;
                    }
                    args.push($parser.$patt()?);
                } else {
                    deprecated_eat!($parser, special, $terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!+, $terminator:expr) => {{
        let mut items = thin_vec![];
        items.push($parser.$patt()?);
        loop {
            if !deprecated_try_eat!($parser, SpecialToken::Comma) {
                deprecated_eat!($parser, special, $terminator);
                break;
            }
            if deprecated_try_eat!($parser, $terminator) {
                break;
            }
            items.push($parser.$patt()?);
        }
        items
    }};
}

#[macro_export]
macro_rules! end {
    ($parser: expr) => {{
        if !$parser.token_stream.is_empty() {
            return err!(format!("expect end"), $parser.token_stream.next_range());
        }
    }};
}
