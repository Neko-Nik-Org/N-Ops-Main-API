use crate::errors::ServiceResult;
use crate::state::SMTP_SETTINGS;
use chrono::Datelike;
use lettre::{
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
    message::{
        Mailbox,
        MultiPart,
        SinglePart,
        Attachment,
        header::ContentType,
    }
};


// N-Ops logo, embedded in every email and shown with <img src="cid:n-ops-logo">
pub const LOGO_PNG: &[u8] = include_bytes!("../../../templates/assets/n-ops-logo.png");
pub const LOGO_CONTENT_ID: &str = "n-ops-logo";


// Internal Email Message Structure
pub struct EmailMessage {
    send_to: Mailbox,
    subject: String,
    plain_body: String,
    html_body: String,
}


/// Keeps the optional notes section ({{#NOTES}} ... {{/NOTES}}) or removes it
pub fn notes_section(template: &str, show: bool) -> String {
    if show {
        return template.replace("{{#NOTES}}", "").replace("{{/NOTES}}", "");
    }
    match (template.find("{{#NOTES}}"), template.find("{{/NOTES}}")) {
        (Some(start), Some(end)) => {
            format!("{}{}", &template[..start], &template[end + "{{/NOTES}}".len()..])
        }
        _ => template.to_string(),
    }
}


impl EmailMessage {
    /// Escapes characters that have a meaning in HTML
    fn escape_html(value: &str) -> String {
        value
            .replace('&', "&amp;")  // Escape ampersand for HTML
            .replace('<', "&lt;")   // Escape less-than for HTML
            .replace('>', "&gt;")   // Escape greater-than for HTML
            .replace('"', "&quot;") // Escape double quote for HTML
            .replace('\'', "&#39;") // Escape single quote for HTML
            .replace('\n', "<br>")  // Replace new lines with <br> for HTML
    }

    /// Creates a new `EmailMessage` with the specified recipient, subject, template, and replacements.
    /// User input is HTML-escaped in the HTML body so it can't add markup or links.
    pub fn new(send_to: Mailbox, subject: String, text_template: &str, html_template: &str, replacements: &[(&str, &str)]) -> Self {
        // Initialize the plain and HTML bodies with the template and apply replacements
        let mut plain_body = text_template.to_string();
        let mut html_body = html_template.to_string();

        // Apply each replacement to both the plain and HTML bodies, escaping HTML in the HTML body
        for &(key, value) in replacements {
            plain_body = plain_body.replace(key, value);
            html_body = html_body.replace(key, &Self::escape_html(value));
        }

        // Replace "{{COPYRIGHT_YEARS}}" always
        let current_year = chrono::Utc::now().year();
        let copyright_years = format!("2024-{}", current_year);

        plain_body = plain_body.replace("{{COPYRIGHT_YEARS}}", &copyright_years);
        html_body = html_body.replace("{{COPYRIGHT_YEARS}}", &copyright_years);

        // Return the constructed `EmailMessage` with the applied replacements
        Self {
            send_to,
            subject,
            plain_body,
            html_body
        }
    }

    /// Sends a basic email asynchronously
    pub async fn send(self) -> ServiceResult<()> {
        // Build the inline logo attachment for the email
        let logo = Attachment::new_inline(LOGO_CONTENT_ID.to_owned()).body(
            LOGO_PNG.to_vec(),
            ContentType::parse("image/png").expect("valid MIME type"),
        );

        // Build the email message using the provided parameters
        let email = Message::builder()
            .from(SMTP_SETTINGS.from_email.clone())
            .to(self.send_to)
            .subject(self.subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(self.plain_body)
                    )
                    .multipart(
                        MultiPart::related()
                            .singlepart(
                                SinglePart::builder()
                                    .header(ContentType::TEXT_HTML)
                                    .body(self.html_body),
                            )
                            .singlepart(logo),
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
}
