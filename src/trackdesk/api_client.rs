use std::time::Duration;

use flurl::FlUrl;

use crate::trackdesk::models::PostbackRequest;

pub struct TrackdeskApiClient {
    url: String,
}

impl TrackdeskApiClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub async fn postback(&self, request: &PostbackRequest) -> Result<(), String> {
        let query_string = serde_qs::to_string(&request).expect("must be valid model");
        let url = format!("{}?{}&status=CONVERSION_STATUS_APPROVED", self.url, query_string);
        let flurl = FlUrl::new(&url).set_timeout(Duration::from_secs(15));
        let result = flurl
            .post(None)
            .await
            .map_err(|e| format!("Failed to send request: {}. Err: {:?}", url, e))?;
        let status_code = result.get_status_code();

        if status_code > 299 {
            return Err(format!(
                "Request with unsucessful status: {}. Status code: {}",
                url, status_code
            ));
        }

        Ok(())
    }
}
