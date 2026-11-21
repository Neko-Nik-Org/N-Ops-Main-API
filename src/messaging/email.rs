mod templates;
mod utils;

use crate::features::users::SessionUser;
use crate::errors::ServiceResult;
use lettre::message::Mailbox;
use utils::EmailMessage;



/// Sends a verification email containing an OTP to the specified session user's email address
pub async fn send_verification_email(session_user: &SessionUser, otp: &str) -> ServiceResult<()> {
    // Build the recipient mailbox for the email
    let send_to_email = Mailbox::new(
        Some(session_user.display_name.clone()),
        session_user.primary_email.clone().parse()?
    );

    // Build the email message
    let email_message = EmailMessage::new(
        send_to_email,
        "Verify Your Email".to_string(),
        templates::VERIFY_EMAIL_TEXT,
        templates::VERIFY_EMAIL_HTML,
        &[
            ("{{DISPLAY_NAME}}", &session_user.display_name),
            ("{{ANTI_PHISHING_CODE}}", &session_user.user_name),
            ("{{OTP_VALIDITY}}", "5 minutes"),
            ("{{OTP}}", &otp),
        ]
    );

    // Send the email using the basic email sending function
    email_message.send().await
}


/// Sends a welcome email to new users after account creation
pub async fn send_welcome_email(display_name: &str, primary_email: &str) -> ServiceResult<()> {
    // Build the recipient mailbox for the email
    let send_to_email = Mailbox::new(
        Some(display_name.to_string()),
        primary_email.parse()?
    );

    // Build the email message
    let email_message = EmailMessage::new(
        send_to_email,
        "Welcome to Our Service".to_string(),
        templates::WELCOME_EMAIL_TEXT,
        templates::WELCOME_EMAIL_HTML,
        &[
            ("{{DISPLAY_NAME}}", display_name),
            ("{{ANTI_PHISHING_CODE}}", display_name),
        ]
    );

    // Send the email using the basic email sending function
    email_message.send().await
}
