#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind};

    async fn my_async_call(url: &str) -> Result<serde_json::Value, Error> {
        let response: reqwest::Response = reqwest::get(url)
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;

        let json_response: serde_json::Value = response
            .json::<serde_json::Value>()
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;

        Ok(json_response)
    }

    #[tokio::test]

}
