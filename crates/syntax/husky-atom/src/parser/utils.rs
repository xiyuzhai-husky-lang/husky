#[macro_export]
macro_rules! try_get {
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
macro_rules! try_eat {
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

    ($parser:expr, SpecialToken::$special:ident) => {{
        let saved_state = $parser.save_state();
        if $parser.special(SpecialToken::$special).is_some() {
            true
        } else {
            $parser.rollback(saved_state);
            false
        }
    }};

    ($parser:expr, "->") => {{
        try_eat!($parser, SpecialToken::LightArrow)
    }};

    ($parser:expr, "(") => {{
        try_eat!($parser, SpecialToken::LPar)
    }};

    ($parser:expr, "<") => {{
        try_eat!($parser, SpecialToken::LAngle)
    }};

    ($parser:expr, ":") => {{
        try_eat!($parser, SpecialToken::Colon)
    }};

    ($parser:expr, "+") => {{
        try_eat!($parser, SpecialToken::Add)
    }};

    ($parser:expr, "'") => {{
        try_eat!($parser, SpecialToken::Lifetime)
    }};

    ($parser:expr, "&") => {{
        try_eat!($parser, SpecialToken::Ambersand)
    }};

    ($parser:expr, "mut") => {{
        try_eat!($parser, token_kind, HuskyTokenKind::Keyword(husky_word::Keyword::Liason(husky_word::LiasonKeyword::Mut)))
    }};

    ($parser:expr, "!!") => {{
        try_eat!($parser, SpecialToken::DoubleExclamation)
    }};

    ($parser:expr, "_") => {{
        try_eat!($parser, elide)
    }};
}

#[macro_export]
macro_rules! get{
    ($parser: expr, $patt:ident) => {{
        let mut saved_state = $parser.save_state();
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
            return err!(format!("expect {:?} after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, $patt:ident?) => {{
        let mut saved_state = $parser.save_state();
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
            return err!(format!("expect {:?} after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};


    ($parser:ident.$patt:ident?) => {{
        if let Some(pattern) = $parser.$patt()? {
            pattern
        } else {
            return err!(
                format!("expect {:?} after it", stringify!($patt)),
                $parser.token_stream.next_range()
            )
        }
    }};

    ($parser:ident.$patt:ident?($($args:expr),*)) => {{
        if let Some(pattern) = this.$patt($args,*)? {
            pattern
        } else {
            return err!(format!("expect {:?} after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};
}

#[macro_export]
macro_rules! eat {
    ($parser:expr, $patt:ident) => {{
        if $parser.$patt().is_none() {
            return err!(format!("expect {:?} after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, $patt:ident, $($args:expr),*) => {{
        if  $parser.$patt($($args),*).is_none() {
            return err!(
                format!("expect {:?} after it", stringify!($patt)),
                $parser.token_stream.next_range()
            )
        }
    }};

    ($parser:expr, $patt:ident?) => {{
        if $parser.$patt()?.is_none() {
            return err!(format!("expect {:?} after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, $patt:ident?, $($args:expr),*) => {{
        if  $parser.$patt($($args),*)?.is_none() {
            return err!(format!("expect {:?} after it", stringify!($patt)), $parser.token_stream.next_range())
        }
    }};

    ($parser:expr, "(") => {{
        eat!($parser, special, SpecialToken::LPar)
    }};

    ($parser:expr, "{") => {{
        eat!($parser, special, SpecialToken::LCurl)
    }};

    ($parser:expr, ":") => {{
        eat!($parser, special, SpecialToken::Colon)
    }};

    ($parser:expr, "=") => {{
        eat!($parser, special, SpecialToken::Assign)
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
            if !try_eat!($parser, SpecialToken::Comma) {
                eat!($parser, special, SpecialToken::$terminator);
                done = true;
                break;
            }
        }
        if !done && !try_eat!($parser, SpecialToken::$terminator) {
            seconds.push($parser.$second_patt()?);
            loop {
                if try_eat!($parser, SpecialToken::Comma) {
                    if try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    seconds.push($parser.$second_patt()?);
                } else {
                    eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        (firsts, seconds)
    }};

    ($parser:expr, $patt:ident, $terminator:ident) => {{
        let mut args = Defaut::default();
        if !try_eat!($parser, SpecialToken::$terminator) {
            args.push(get!($parser, $patt?));
            loop {
                if try_eat!($parser, SpecialToken::Comma) {
                    if try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    args.push(get!($parser, $patt?));
                } else {
                    eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!, $terminator:ident) => {{
        let mut args = vec![];
        if !try_eat!($parser, SpecialToken::$terminator) {
            args.push($parser.$patt()?);
            loop {
                if try_eat!($parser, SpecialToken::Comma) {
                    if try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    args.push($parser.$patt()?);
                } else {
                    eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!+, $terminator:ident) => {{
        let mut items = vec![$parser.$patt()?];
        loop {
            if !try_eat!($parser, SpecialToken::Comma) {
                eat!($parser, special, SpecialToken::$terminator);
                break;
            }
            if try_eat!($parser, SpecialToken::$terminator) {
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

#[macro_export]
macro_rules! thin_comma_list {
    ($parser:expr, $first_patt:ident?, $second_patt:ident!, $terminator:ident) => {{
        let mut firsts = thin_vec![];
        let mut seconds = thin_vec![];
        let mut done = false;
        while let Some(item) = try_get!($parser, $first_patt?) {
            firsts.push(item);
            if !try_eat!($parser, SpecialToken::Comma) {
                eat!($parser, special, SpecialToken::$terminator);
                done = true;
                break;
            }
        }
        if !done && !try_eat!($parser, SpecialToken::$terminator) {
            seconds.push($parser.$second_patt()?);
            loop {
                if try_eat!($parser, SpecialToken::Comma) {
                    if try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    seconds.push($parser.$second_patt()?);
                } else {
                    eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        (firsts, seconds)
    }};

    ($parser:expr, $patt:ident, $terminator:ident) => {{
        let mut args = thin_vec![];
        if !try_eat!($parser, SpecialToken::$terminator) {
            args.push(get!($parser, $patt?));
            loop {
                if try_eat!($parser, SpecialToken::Comma) {
                    if try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    args.push(get!($parser, $patt?));
                } else {
                    eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!, $terminator:ident) => {{
        let mut args = thin_vec![];
        if !try_eat!($parser, SpecialToken::$terminator) {
            args.push($parser.$patt()?);
            loop {
                if try_eat!($parser, SpecialToken::Comma) {
                    if try_eat!($parser, SpecialToken::$terminator) {
                        break;
                    }
                    args.push($parser.$patt()?);
                } else {
                    eat!($parser, special, SpecialToken::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($parser:expr, $patt:ident!+, $terminator:ident) => {{
        let mut items = thin_vec![];
        items.push($parser.$patt()?);
        loop {
            if !try_eat!($parser, SpecialToken::Comma) {
                eat!($parser, special, SpecialToken::$terminator);
                break;
            }
            if try_eat!($parser, SpecialToken::$terminator) {
                break;
            }
            items.push($parser.$patt()?);
        }
        items
    }};

    ($parser:expr, $patt:ident!, ")") => {{
        thin_comma_list!($parser, $patt!, RPar)
    }};

    ($parser:expr, $patt:ident!, "|") => {{
        thin_comma_list!($parser, $patt!, Vertical)
    }};

    ($parser:expr, $patt:ident!+, ">") => {{
        thin_comma_list!($parser, $patt!+, RAngle)
    }};
}

#[macro_export]
macro_rules! end {
    ($parser: expr) => {{
        if !$parser.token_stream.empty() {
            return err!(format!("expect end"), $parser.token_stream.next_range());
        }
    }};
}
