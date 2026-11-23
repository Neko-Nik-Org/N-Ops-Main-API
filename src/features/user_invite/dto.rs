use crate::errors::{ServiceResult, ServiceError};
use serde::Deserialize;


type ChronoUtc = chrono::DateTime<chrono::Utc>;



#[derive(Deserialize)]
pub struct ListInvitesDto {
    pub email: Option<String>,    // Filter invites by email (optional)
    pub limit: i64,
    pub offset: i64
}


#[derive(Deserialize)]
pub struct NewInviteDto {
    pub invite_to_email: String,    // Send invite to this email
    pub invitation_notes: String,

    pub role_id: i64,
    pub access_restriction_policy_id: Option<i64>,
    pub expiration_datetime: ChronoUtc
}


// ------- Implementations ------- //


impl NewInviteDto {
    pub fn validate(&self) -> ServiceResult<()> {
        if self.invite_to_email.trim().is_empty() {
            return Err(ServiceError::BadRequest("Invite to email cannot be empty".into()));
        }

        // Can not have any uppercase letters in the email
        if self.invite_to_email.chars().any(|c| c.is_uppercase()) {
            return Err(ServiceError::BadRequest("Invite to email cannot contain uppercase letters".into()));
        }

        // Max of 21 Days for expiration datetime (minus 10 minutes buffer)
        let max_expiration = chrono::Utc::now() + chrono::Duration::days(21) - chrono::Duration::minutes(10);
        if self.expiration_datetime > max_expiration {
            return Err(ServiceError::BadRequest("Expiration datetime cannot be more than 21 days from now".into()));
        }

        // Ensure expiration datetime is at least 1 hour from now
        let min_expiration = chrono::Utc::now() + chrono::Duration::hours(1) - chrono::Duration::minutes(5);
        if self.expiration_datetime < min_expiration {
            return Err(ServiceError::BadRequest("Expiration datetime must be at least 1 hour from now".into()));
        }

        Ok(())
    }
}
