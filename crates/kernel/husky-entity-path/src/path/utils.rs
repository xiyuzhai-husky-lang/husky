macro_rules! debug_with_db_fmt {
    ($slf: ident, $f: ident, $db: ident, $ty_str: literal) => {{
        let data = $slf.data($db);
        $f.debug_tuple($ty_str)
            .field_with(|f| {
                f.write_str("`")?;
                data.show_aux(f, $db)?;
                f.write_str("`")
            })
            .field(&data.item_kind.debug($db))
            .finish()?;
        // this is due to rustc borrow checker's wierdness
        Ok(())
    }};
}

pub(crate) use debug_with_db_fmt;
