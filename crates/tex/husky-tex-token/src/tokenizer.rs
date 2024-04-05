use husky_text_protocol::char_iter::TextCharIter;

pub struct TexTokenizer<'a> {
    db: &'a ::salsa::Db,
    chars: TextCharIter<'a>,
}
