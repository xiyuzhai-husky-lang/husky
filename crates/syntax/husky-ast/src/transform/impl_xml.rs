use super::*;
use crate::*;
use husky_word::IdentPairDict;

impl<'a> AstTransformer<'a> {
    pub(crate) fn parse_xml_expr(
        &mut self,
        token_group: &[HuskyToken],
    ) -> AstResultArc<RawXmlExpr> {
        msg_once!("todo: parse children");
        let variant = match token_group[0].kind {
            HuskyTokenKind::Special(SpecialToken::LAngle) => {
                if token_group.len() < 2 {
                    return err!(format!("expect xml"), token_group.text_range());
                }
                match token_group.last().unwrap().kind {
                    HuskyTokenKind::Special(SpecialToken::XmlKet) => (),
                    _ => return err!(format!("expect `/>`"), token_group[0].range),
                }
                if token_group.len() == 2 {
                    todo!()
                }
                let ident = identify_token!(self, &token_group[1], SemanticTokenKind::XmlTagKind);
                RawXmlExprVariant::Tag {
                    ident: ident.ident,
                    props: self.parse_xml_props(&token_group[2..(token_group.len() - 1)])?,
                }
            }
            _ => RawXmlExprVariant::Value(self.parse_expr(token_group)?),
        };
        Ok(Arc::new(RawXmlExpr {
            variant,
            range: token_group.text_range(),
        }))
    }

    fn parse_xml_props(&mut self, tokens: &[HuskyToken]) -> AstResult<IdentPairDict<RawExprIdx>> {
        self.parse_atoms(tokens, |parser| parser.xml_props())?
            .into_iter()
            .map(
                |(ranged_ident, token_range)| -> AstResult<(CustomIdentifier, RawExprIdx)> {
                    self.push_abs_semantic_token(husky_token::AbsSemanticToken::new(
                        SemanticTokenKind::Parameter,
                        ranged_ident.range,
                    ));
                    Ok((ranged_ident.ident, self.parse_expr(&tokens[token_range])?))
                },
            )
            .collect()
    }
}
