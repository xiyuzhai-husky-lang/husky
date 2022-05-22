use super::*;
use crate::*;
use atom::symbol::{Symbol, SymbolKind};
use entity_route::{EntityRoute, EntityRouteKind};
use text::TextRange;
use word::IdentPairDict;

impl<'a> AstTransformer<'a> {
    pub(crate) fn parse_xml_expr(&mut self, token_group: &[Token]) -> AstResultArc<RawXmlExpr> {
        msg_once!("todo: parse children");
        if token_group.len() < 2 {
            return err!(format!("expect xml"), token_group.text_range());
        }
        match token_group[0].kind {
            TokenKind::Special(Special::LAngle) => (),
            _ => return err!(format!("expect `<`"), token_group[0].range),
        }
        match token_group.last().unwrap().kind {
            TokenKind::Special(Special::XmlKet) => (),
            _ => return err!(format!("expect `/>`"), token_group[0].range),
        }
        if token_group.len() == 2 {
            todo!()
        }
        let ident = identify_token!(self, &token_group[1], SemanticTokenKind::XmlKind);
        Ok(Arc::new(RawXmlExpr {
            ident: ident.ident,
            props: self.parse_xml_props(&token_group[2..(token_group.len() - 1)])?,
            range: token_group.text_range(),
        }))
    }

    fn parse_xml_props(&mut self, tokens: &[Token]) -> AstResult<IdentPairDict<RawExprIdx>> {
        self.parse_atoms(tokens, |parser| parser.xml_props())?
            .into_iter()
            .map(
                |(ident, token_range)| -> AstResult<(CustomIdentifier, RawExprIdx)> {
                    Ok((ident, self.parse_expr(&tokens[token_range])?))
                },
            )
            .collect()
        // let mut props = IdentPairDict::<RawExprIdx>::default();
        // let mut iter = tokens.iter().enumerate();
        // while let Some((_, ident_token)) = iter.next() {
        //     let ident = identify_token!(self, ident_token, SemanticTokenKind::Field);
        //     let (_, lcurl_token) = iter.next();
        //     expect_token_kind!(lcurl_token, Special::LCurl);
        //     match ident_token.kind {
        //         TokenKind::Special(Special::LCurl) => (),
        //         _ => todo!(),
        //     }
        //     todo!()
        // }
        // Ok(props)
    }
}
