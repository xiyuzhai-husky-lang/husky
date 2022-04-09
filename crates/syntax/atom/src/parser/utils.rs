macro_rules! try_get {
    ($this:expr, $patt:ident) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt() {
         Some(pattern)
     } else {
         $this.rollback(saved_stream);
         None
     }
    }};

    ($this:expr, $patt:ident?) => {{
        let saved_stream = $this.save_stream();
        if let Some(pattern) = $this.$patt()? {
            Some(pattern)
        } else {
            $this.rollback(saved_stream);
            None
        }
       }};

    ($this:expr, $patt:ident, $($args:expr),*) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt($($args),*) {
         Some(pattern)
     } else {
         $this.rollback(saved_stream);
         None
     }
    }};

    ($this:expr, $patt:ident?, $($args:expr),*) => {{
        let saved_stream = $this.save_stream();
        if let Some(pattern) = $this.$patt($($args),*)? {
            Some(pattern)
        } else {
            $this.rollback(saved_stream);
            None
        }
    }};

    ($this:expr, "_") => {{
        try_get!($this, elide)
    }};
}
pub(crate) use try_get;

macro_rules! next_matches {
    ($this:expr, $patt:ident) => {{
     let saved_stream = $this.save_stream();
     if $this.$patt().is_some() {
         true
     } else {
         $this.rollback(saved_stream);
         false
     }
    }};

    ($this:expr, $patt:ident, $($args:expr),+) => {{
     let saved_stream = $this.save_stream();
     if $this.$patt($($args),*).is_some() {
         true
     } else {
         $this.rollback(saved_stream);
         false
     }
    }};

    ($this:expr, $patt:ident?, $($args:expr),+) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt($($args),*)? {
         true
     } else {
         $this.rollback(saved_stream);
         false
     }
    }};

    ($this:expr, $patt:ident?, $($args:expr),+) => {{
        let saved_stream = $this.save_stream();
        if let Some(pattern) = $this.$patt($($args),*)? {
            true
        } else {
            $this.rollback(saved_stream);
            false
        }
    }};

    ($this:expr, Special::$special:ident) => {{
        let saved_stream = $this.save_stream();
        if $this.special(Special::$special).is_some() {
            true
        } else {
            $this.rollback(saved_stream);
            false
        }
    }};

    ($this:expr, "->") => {{
        next_matches!($this, Special::LightArrow)
    }};

    ($this:expr, "(") => {{
        next_matches!($this, Special::LPar)
    }};

    ($this:expr, "<") => {{
        next_matches!($this, Special::LAngle)
    }};

    ($this:expr, ":") => {{
        next_matches!($this, Special::Colon)
    }};

    ($this:expr, "+") => {{
        next_matches!($this, Special::Add)
    }};

    ($this:expr, "'") => {{
        next_matches!($this, Special::Lifetime)
    }};

    ($this:expr, "_") => {{
        next_matches!($this, elide)
    }};
}
pub(crate) use next_matches;

macro_rules! get{
    ($this:expr, $patt:ident) => {{
        let mut saved_stream = $this.save_stream();
        if let Some(pattern) = $this.$patt() {
            pattern
        } else {
            return err!(
                format!("expect {} after it, but get {{{:?}}} instead",
                    stringify!($patt),
                    saved_stream.next()
                ),
                $this.stream.pop_range()
            )
        }
    }};

    ($this:expr, $patt:ident, $($args:expr),*) => {{
        if let Some(pattern) = this.$patt($args,*) {
            pattern
        } else {
            return err!(format!("expect {:?} after it", stringify!($patt)), $this.stream.pop_range())
        }
    }};

    ($this:expr, $patt:ident?) => {{
        let mut saved_stream = $this.save_stream();
        if let Some(pattern) = $this.$patt()? {
            pattern
        } else {
            return err!(
                    format!("expect {} after it, but get {{{:?}}} instead",
                        stringify!($patt),
                        saved_stream.next()
                    ),
                    $this.stream.pop_range()
                )
        }
    }};

    ($this:expr, $patt:ident?, $($args:expr),*) => {{
        if let Some(pattern) = this.$patt($args,*)? {
            pattern
        } else {
            return err!(format!("expect {:?} after it", stringify!($patt)), $this.stream.pop_range())
        }
    }};


    ($this:ident.$patt:ident?) => {{
        if let Some(pattern) = $this.$patt()? {
            pattern
        } else {
            return err!(
                format!("expect {:?} after it", stringify!($patt)),
                $this.stream.pop_range()
            )
        }
    }};

    ($this:ident.$patt:ident?($($args:expr),*)) => {{
        if let Some(pattern) = this.$patt($args,*)? {
            pattern
        } else {
            return err!(format!("expect {:?} after it", stringify!($patt)), $this.stream.pop_range())
        }
    }};
}
pub(crate) use get;

macro_rules! no_look_pass {
    ($this:expr, $patt:ident) => {{
        if $this.$patt().is_none() {
            return err!(format!("expect {:?} after it", stringify!($patt)), $this.stream.pop_range())
        }
    }};

    ($this:expr, $patt:ident, $($args:expr),*) => {{
        if  $this.$patt($($args),*).is_none() {
            return err!(
                format!("expect {:?} after it", stringify!($patt)),
                $this.stream.pop_range()
            )
        }
    }};

    ($this:expr, $patt:ident?) => {{
        if $this.$patt()?.is_none() {
            return err!(format!("expect {:?} after it", stringify!($patt)), $this.stream.pop_range())
        }
    }};

    ($this:expr, $patt:ident?, $($args:expr),*) => {{
        if  $this.$patt($($args),*)?.is_none() {
            return err!(format!("expect {:?} after it", stringify!($patt)), $this.stream.pop_range())
        }
    }};

    ($this:expr, "(") => {{
        no_look_pass!($this, special, Special::LPar)
    }};

    ($this:expr, ":") => {{
        no_look_pass!($this, special, Special::Colon)
    }};
}
pub(crate) use no_look_pass;

macro_rules! comma_list {
    ($this:expr, $first_patt:ident?, $second_patt:ident!, $terminator:ident) => {{
        let mut firsts = Vec::new();
        let mut seconds = Vec::new();
        let mut done = false;
        while let Some(item) = try_get!($this, $first_patt?) {
            firsts.push(item);
            if !next_matches!($this, Special::Comma) {
                no_look_pass!($this, special, Special::$terminator);
                done = true;
                break;
            }
        }
        if !done && !next_matches!($this, Special::$terminator) {
            seconds.push($this.$second_patt()?);
            loop {
                if next_matches!($this, Special::Comma) {
                    if next_matches!($this, Special::$terminator) {
                        break;
                    }
                    seconds.push($this.$second_patt()?);
                } else {
                    no_look_pass!($this, special, Special::$terminator);
                    break;
                }
            }
        }
        (firsts, seconds)
    }};

    ($this:expr, $patt:ident, $terminator:ident) => {{
        let mut args = Vec::new();
        if !next_matches!($this, Special::$terminator) {
            args.push(get!($this, $patt?));
            loop {
                if next_matches!($this, Special::Comma) {
                    if next_matches!($this, Special::$terminator) {
                        break;
                    }
                    args.push(get!($this, $patt?));
                } else {
                    no_look_pass!($this, special, Special::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($this:expr, $patt:ident!, $terminator:ident) => {{
        let mut args = Vec::new();
        if !next_matches!($this, Special::$terminator) {
            args.push($this.$patt()?);
            loop {
                if next_matches!($this, Special::Comma) {
                    if next_matches!($this, Special::$terminator) {
                        break;
                    }
                    args.push($this.$patt()?);
                } else {
                    no_look_pass!($this, special, Special::$terminator);
                    break;
                }
            }
        }
        args
    }};

    ($this:expr, $patt:ident!+, $terminator:ident) => {{
        let mut items = vec![$this.$patt()?];
        loop {
            if !next_matches!($this, Special::Comma) {
                no_look_pass!($this, special, Special::$terminator);
                break;
            }
            if next_matches!($this, Special::$terminator) {
                break;
            }
            items.push($this.$patt()?);
        }
        items
    }};

    ($this:expr, $patt:ident!, ")") => {{
        comma_list!($this, $patt!, RPar)
    }};

    ($this:expr, $patt:ident!, "|") => {{
        comma_list!($this, $patt!, Vertical)
    }};

    ($this:expr, $patt:ident!+, ">") => {{
        comma_list!($this, $patt!+, RAngle)
    }};
}
pub(crate) use comma_list;
