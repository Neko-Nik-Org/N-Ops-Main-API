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



/// Sends a basic email asynchronously
pub async fn send_basic_email(send_to: Mailbox, subject: String, plain_body: String, html_body: String) -> ServiceResult<()> {
    // Build the email message using the provided parameters
    let email = Message::builder()
        .from(SMTP_SETTINGS.from_email.clone())
        .to(send_to)
        .subject(subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_PLAIN)
                        .body(plain_body)
                )
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_body)
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
