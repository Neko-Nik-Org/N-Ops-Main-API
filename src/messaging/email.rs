use crate::features::users::SessionUser;
use crate::errors::ServiceResult;
use crate::state::SMTP_SETTINGS;
use lettre::{
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
    message::{
        Mailbox,
        MultiPart,
        SinglePart,
        header::ContentType,
    }
};


// Email templates for various email types

const VERIFY_EMAIL_HTML: &str = include_str!("../../templates/verify_email/email_otp.html");

const VERIFY_EMAIL_TEXT: &str = include_str!("../../templates/verify_email/email_otp.txt");

const WELCOME_EMAIL_HTML: &str = include_str!("../../templates/initial_email/welcome_email.html");

const WELCOME_EMAIL_TEXT: &str = include_str!("../../templates/initial_email/welcome_email.txt");


struct EmailMessage {
    send_to: Mailbox,
    subject: String,
    plain_body: String,
    html_body: String,
}


/// Sends a basic email asynchronously
async fn send_basic_email(email_message: EmailMessage) -> ServiceResult<()> {
    // Build the email message using the provided parameters
    let email = Message::builder()
        .from(SMTP_SETTINGS.from_email.clone())
        .to(email_message.send_to)
        .subject(email_message.subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(email_message.plain_body)
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(email_message.html_body)
                )
        )?;

    // Create the SMTP mailer using the settings from SMTP_SETTINGS
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(
        &SMTP_SETTINGS.host
    )?
    .credentials(SMTP_SETTINGS.credentials.clone())
    .build();

    // Send the email using the SMTP mailer
    mailer.send(email).await?;

    Ok(())
}


/// Sends a verification email containing an OTP to the specified session user's email address
pub async fn send_verification_email(session_user: &SessionUser, otp: &str) -> ServiceResult<()> {
    // Prepare the HTML and plain text bodies for the verification email
    let html_body = VERIFY_EMAIL_HTML
        .replace("{{DISPLAY_NAME}}", &session_user.display_name)
        .replace("{{OTP}}", &otp)
        .replace("2026", "2026");

    // Prepare the plain text body for the verification email
    let plain_body = VERIFY_EMAIL_TEXT
        .replace("{{DISPLAY_NAME}}", &session_user.display_name)
        .replace("{{OTP}}", &otp);

    // Build the recipient mailbox for the email
    let send_to_email = Mailbox::new(
        Some(session_user.display_name.clone()),
        session_user.primary_email.clone().parse()?
    );

    // Build the email message using the recipient mailbox and prepared bodies
    let email_message = EmailMessage {
        send_to: send_to_email,
        subject: "Verify Your Email".to_string(),
        plain_body,
        html_body,
    };

    // Send the email using the basic email sending function
    send_basic_email(email_message).await
}
