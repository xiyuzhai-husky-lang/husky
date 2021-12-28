macro_rules! try_get {
    ($this:ident, $patt:ident) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt()? {
         Some(pattern)
     } else {
         $this.rollback(saved_stream);
         None
     }
    }};

    ($this:ident, $patt:ident, $($args:expr),*) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt($($args),*) {
         Some(pattern)
     } else {
         $this.rollback(saved_stream);
         None
     }
    }};

    ($this:ident, $patt:ident?, $($args:expr),*) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt($($args),*)? {
         Some(pattern)
     } else {
         $this.rollback(saved_stream);
         None
     }
    }}
}
pub(crate) use try_get;

macro_rules! next_matches {
    ($this:ident, $patt:ident) => {{
     let saved_stream = $this.save_stream();
     if $this.$patt().is_some() {
         true
     } else {
         $this.rollback(saved_stream);
         false
     }
    }};

    ($this:ident, $patt:ident, $($args:expr),+) => {{
     let saved_stream = $this.save_stream();
     if $this.$patt($($args),*).is_some() {
         true
     } else {
         $this.rollback(saved_stream);
         false
     }
    }};

    ($this:ident, $patt:ident?, $($args:expr),+) => {{
     let saved_stream = $this.save_stream();
     if let Some(pattern) = $this.$patt($($args),*)? {
         true
     } else {
         $this.rollback(saved_stream);
         false
     }
    }};

    ($this:ident, $patt:ident?, $($args:expr),+) => {{
        let saved_stream = $this.save_stream();
        if let Some(pattern) = $this.$patt($($args),*)? {
            true
        } else {
            $this.rollback(saved_stream);
            false
        }
    }};

    ($this:ident, Special::$special:ident) => {{
        let saved_stream = $this.save_stream();
        if $this.special(Special::$special).is_some() {
            true
        } else {
            $this.rollback(saved_stream);
            false
        }
    }};

    ($this:ident, "->") => {{
        next_matches!($this, Special::LightArrow)
    }};

    ($this:ident, "(") => {{
        next_matches!($this, Special::LPar)
    }};

    ($this:ident, "<") => {{
        next_matches!($this, Special::LAngle)
    }};

    ($this:ident, ":") => {{
        next_matches!($this, Special::Colon)
    }};

    ($this:ident, "+") => {{
        next_matches!($this, Special::Add)
    }};
}
pub(crate) use next_matches;

macro_rules! get{
    ($this:ident, $patt:ident) => {{
        if let Some(pattern) = $this.$patt() {
            pattern
        } else {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: common::src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};

    ($this:ident, $patt:ident, $($args:expr),*) => {{
        if let Some(pattern) = this.$patt($args,*) {
            pattern
        } else {
            return Err(AtomError{range:$this.stream.pop_range(),
                    src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};

    ($this:ident, $patt:ident?) => {{
        if let Some(pattern) = $this.$patt()? {
            pattern
        } else {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};

    ($this:ident, $patt:ident?, $($args:expr),*) => {{
        if let Some(pattern) = this.$patt($args,*)? {
            pattern
        } else {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};


    ($this:ident.$patt:ident?) => {{
        if let Some(pattern) = $this.$patt()? {
            pattern
        } else {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};

    ($this:ident.$patt:ident?($($args:expr),*)) => {{
        if let Some(pattern) = this.$patt($args,*)? {
            pattern
        } else {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};
}
pub(crate) use get;

macro_rules! no_look_pass{
    ($this:ident, $patt:ident) => {{
        if $this.$patt().is_none() {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};

    ($this:ident, $patt:ident, $($args:expr),*) => {{
        if  $this.$patt($($args),*).is_none() {
            return Err(AtomError{range:$this.stream.pop_range(),
                    src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
    }};

    ($this:ident, $patt:ident?) => {{
        if $this.$patt()?.is_none() {
            return Err(AtomError{range:$this.stream.pop_range(),
                 src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
        }
       }};

    ($this:ident, $patt:ident?, $($args:expr),*) => {{
    if  $this.$patt($($args),*)?.is_none() {
        return Err(AtomError{range:$this.stream.pop_range(),
                src: src!(), kind: format!("expect {:?} after it", stringify!($patt)).into()})
    }
    }};
}
pub(crate) use no_look_pass;

macro_rules! comma_list {
    ($this:ident, $patt:ident, $terminator:ident) => {{
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

    ($this:ident, $patt:ident!, $terminator:ident) => {{
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

    ($this:ident, $patt:ident!+, $terminator:ident) => {{
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

    ($this:ident, $patt:ident!, ")") => {{
        comma_list!($this, $patt!, RPar)
    }};

    ($this:ident, $patt:ident!, "|") => {{
        comma_list!($this, $patt!, Vertical)
    }};

    ($this:ident, $patt:ident!+, ">") => {{
        comma_list!($this, $patt!+, RAngle)
    }};
}
pub(crate) use comma_list;
