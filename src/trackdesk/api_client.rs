use crate::trackdesk::models::PostbackRequest;
use flurl::FlUrl;
use std::time::Duration;

pub struct TrackdeskApiClient {}

impl TrackdeskApiClient {
    pub async fn postback(&self, request: &PostbackRequest) -> Result<(), String> {
        let query_string = serde_qs::to_string(&request).expect("must be valid model");
        let url = format!("https://tradelocker.trackdesk.com/tracking/conversion/v1?status=CONVERSION_STATUS_APPROVED&{}", query_string);
        let flurl = FlUrl::new(&url).set_timeout(Duration::from_secs(15));
        let result = flurl
            .get()
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
