mod templates;
mod utils;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use crate::features::users::SessionUser;
use crate::state::ORG_INVITE_SETTINGS;
use crate::errors::ServiceResult;
use lettre::message::Mailbox;
use utils::EmailMessage;

type ChronoUtc = chrono::DateTime<chrono::Utc>;


/// Sends a verification email containing an OTP to the specified session user's email address
pub async fn send_verification_email(session_user: &SessionUser, otp: &str, anti_phishing_code: &str) -> ServiceResult<()> {
    // Build the recipient mailbox for the email
    let send_to_email = Mailbox::new(
        Some(session_user.display_name.clone()),
        session_user.primary_email.clone().parse()?
    );

    // Build the email message
    let email_message = EmailMessage::new(
        send_to_email,
        format!("N-Ops: Verify Your Email from {}", session_user.organization_name),
        templates::VERIFY_EMAIL_TEXT,
        templates::VERIFY_EMAIL_HTML,
        &[
            ("{{DISPLAY_NAME}}", &session_user.display_name),
            ("{{ANTI_PHISHING_CODE}}", anti_phishing_code),
            ("{{OTP_VALIDITY}}", "5 minutes"),
            ("{{OTP}}", &otp),
        ]
    );

    // Send the email using the basic email sending function
    email_message.send().await
}


/// Sends a welcome email to new users after account creation
pub async fn send_welcome_email(org_name: &str, display_name: &str, primary_email: &str, anti_phishing_code: &str) -> ServiceResult<()> {
    // Build the recipient mailbox for the email
    let send_to_email = Mailbox::new(
        Some(display_name.to_string()),
        primary_email.parse()?
    );

    // Build the email message
    let email_message = EmailMessage::new(
        send_to_email,
        format!("N-Ops: Welcome to {}", org_name),
        templates::WELCOME_EMAIL_TEXT,
        templates::WELCOME_EMAIL_HTML,
        &[
            ("{{DISPLAY_NAME}}", display_name),
            ("{{ANTI_PHISHING_CODE}}", anti_phishing_code),
        ]
    );

    // Send the email using the basic email sending function
    email_message.send().await
}


/// Sends an invite email to the specified email address with the provided invitation code
pub async fn send_invite_email(
    invite_to_email: &str,
    invitation_notes: &str,
    expiration_datetime: &ChronoUtc,
    invitation_code: &str,
    invited_by: &str,
    org_name: &str
) -> ServiceResult<()> {
    // Build the recipient mailbox for the email
    let send_to_email = Mailbox::new(
        None,
        invite_to_email.parse()?
    );

    // Build the invitation details as a JSON object
    let invitation_details = serde_json::json!({
        "invitation_code": invitation_code,
        "email": invite_to_email,
        "invited_by": invited_by,
        "org_name": org_name
    });

    // Build the invitation URL
    let invitation_url = format!(
        "{}/invite?code={}",
        ORG_INVITE_SETTINGS.base_url,
        // Build the code by encoding the invitation details as a base64 string
        BASE64_STANDARD.encode(invitation_details.to_string())
    );

    let expires_at = expiration_datetime.format("%-d %b %Y, %H:%M UTC").to_string();
    let notes: String = invitation_notes
        .trim()
        .replace("\r\n", "\n")
        .chars()
        .take(500)
        .collect();
    let has_notes = !notes.is_empty();

    // Determine if there are any invitation notes to include in the email
    let invite_email_html = templates::INVITE_EMAIL_HTML.to_string();
    let invite_email_text = templates::INVITE_EMAIL_TEXT.to_string();

    // Remove the optional notes section if there are no notes ({{#NOTES}} ... {{/NOTES}})
    let invite_email_text = utils::notes_section(&invite_email_text, has_notes);
    let invite_email_html = utils::notes_section(&invite_email_html, has_notes);

    // Build the email message
    let email_message = EmailMessage::new(
        send_to_email,
        format!("N-Ops: You're Invited to Join {}", org_name),
        &invite_email_text,
        &invite_email_html,
        &[
            ("{{ORG_NAME}}", org_name),
            ("{{INVITED_BY}}", invited_by),
            ("{{INVITE_URL}}", &invitation_url),
            ("{{EXPIRES_AT}}", &expires_at),
            ("{{INVITATION_NOTES}}", &notes),
        ]
    );

    // Send the email using the basic email sending function
    email_message.send().await
}
