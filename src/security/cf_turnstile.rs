use crate::errors::{ServiceResult, ServiceError};
use crate::state::CLOUDFLARE_TURNSTILE_SETTINGS;



/// Verifies the Cloudflare Turnstile CAPTCHA response token
pub async fn verify_turnstile_response(response_token: &str) -> ServiceResult<()> {
    let client = reqwest::Client::new();

    let response = client.post(&CLOUDFLARE_TURNSTILE_SETTINGS.api_url)
        .header("content-type", "application/json")
        .body(serde_json::json!({
            "response": response_token,
            "secret": &CLOUDFLARE_TURNSTILE_SETTINGS.secret_key
        }).to_string())
        .send()
        .await?;

    if response.status().is_success() {
        // Check if the response body JSON has "success": true
        let response_json: serde_json::Value = response.json().await?;
        let success_key = response_json
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if success_key {
            Ok(())
        } else {
            tracing::error!("Failed to verify CAPTCHA response: {:?}", response_json);

            Err(ServiceError::BadRequest("Failed to verify CAPTCHA response".into()))
        }
    } else {
        let error_text = response.text().await?;

        tracing::error!("Error verifying CAPTCHA response: {}", error_text);

        Err(ServiceError::BadRequest("Error verifying CAPTCHA response".into()))
    }
}
