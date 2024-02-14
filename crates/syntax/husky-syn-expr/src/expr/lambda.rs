use super::*;
use parsec::{HasStreamState, IsStreamParser};

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_lambda(
        &mut self,
        lvert_regional_token_idx: RegionalTokenIdx,
    ) -> SynExprResult<SynExprData> {
        let parameters: PunctuatedSmallList<
            LambdaParameterSyndicate,
            CommaRegionalToken,
            SynExprError,
            true,
            3,
        > = self.try_parse()?;
        let rvert_regional_token =
            self.try_parse_expected(OriginalSynExprError::ExpectedRvertForLambda)?;
        let light_arrow_token: Option<LightArrowRegionalToken> = self.try_parse_option()?;
        let return_ty = match light_arrow_token {
            Some(light_arrow_token) => {
                let return_ty = self.parse_expr_expected(
                    None,
                    OriginalSynExprError::ExpectedReturnTypeAfterLightArrowForLambda,
                )?;
                let eq_token = self.try_parse_expected(
                    OriginalSynExprError::ExpectedEqTokenAfterReturnTypeForLambda,
                )?;
                Some((light_arrow_token, return_ty, eq_token))
            }
            None => None,
        };
        let body =
            self.parse_expr_expected(None, OriginalSynExprError::ExpectedBodyExprForLambda)?;
        let access_end = RegionalTokenIdxRangeEnd::new_lambda_variables_access_end(self.state());
        for param in parameters.elements() {
            match param {
                LambdaParameterSyndicate::Simple { variables, .. } => {
                    for var in variables {
                        self.context_mut()
                            .set_lambda_variable_access_end(var, access_end)
                    }
                }
            }
        }
        Ok(SynExprData::Lambda {
            ritchie_kind_regional_token_idx: None,
            lvert_regional_token_idx,
            parameters,
            rvert_regional_token,
            return_ty,
            body,
        })
    }
}
