macro_rules! item_debug_with_db_fmt {
    ($slf: ident, $f: ident, $ty_str: literal, $db: ident) => {{
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

pub(crate) use item_debug_with_db_fmt;
