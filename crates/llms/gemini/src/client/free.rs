use super::*;

impl<'db> GeminiClient<'db> {
    pub fn generate_on_free(
        &self,
        model: GeminiModel,
        request: GeminiRequest,
    ) -> GeminiResult<GeminiResponse> {
        let min_usage = request.min_usage();
        let response = self.caches[model].get_or_call(
            attached_seed(),
            request,
            |request| -> GeminiResult<GeminiResponse> {
                match try_call_gemini::<GeminiResult<GeminiResponse>>(min_usage, || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(self.generate_on_free_aux(model, request))
                })? {
                    Ok(result) => match result {
                        Ok(s) => Ok(s),
                        Err(e) => Err(e),
                    },
                    Err(e) => todo!(),
                }
            },
        )?;

        Ok(response)
    }

    async fn generate_on_free_aux(
        &self,
        model: GeminiModel,
        request: &GeminiRequest,
    ) -> (usize, GeminiResult<GeminiResponse>) {
        loop {
            match self.generate_on_free_step(model, request).await {
                Some(result) => return result,
                None => continue,
            }
        }
    }

    async fn generate_on_free_step(
        &self,
        model: GeminiModel,
        request: &GeminiRequest,
    ) -> Option<(usize, GeminiResult<GeminiResponse>)> {
        let mut usage = 0;
        let raw_request: GeminiRawRequest = request.into();
        let api_key = match self.api_key() {
            Ok(api_key) => api_key,
            Err(e) => return Some((usage, Err(e))),
        };
        let response = match self
            .client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                model.url_name(),
                api_key
            ))
            .json(&raw_request)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Some((usage, Err(e.into()))),
        };

        let response_bytes = match response.bytes().await {
            Ok(bytes) => {
                usage += POST_CALL_USAGE_MULTIPLIER * bytes.len();
                bytes
            }
            Err(e) => return Some((usage, Err(e.into()))),
        };

        match parse_response_result(&response_bytes) {
            Ok(resp_result) => match resp_result {
                Ok(resp) => {
                    usage +=
                        POST_CALL_USAGE_MULTIPLIER * resp.candidates[0].content.parts[0].text.len();
                    Some((usage, Ok((resp, request).into())))
                }
                Err(e) => match e.error.status.as_str() {
                    "RESOURCE_EXHAUSTED" => {
                        tracing::info!("RESOURCE_EXHAUSTED, retrying in {:?}...", self.retry_delay);
                        tokio::time::sleep(tokio::time::Duration::from(self.retry_delay)).await;
                        None
                    }
                    _ => todo!(),
                },
            },
            Err(e) => Some((usage, Err(e))),
        }
    }
}
