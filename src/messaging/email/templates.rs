// Verification email - one-time code to verify the user's email address
pub const VERIFY_EMAIL_HTML: &str = include_str!("../../../templates/verify_email/email_otp.html");
pub const VERIFY_EMAIL_TEXT: &str = include_str!("../../../templates/verify_email/email_otp.txt");


// Verification success - confirms the address is now verified
pub const VERIFY_EMAIL_SUCCESS_HTML: &str = include_str!("../../../templates/verify_email/email_success.html");
pub const VERIFY_EMAIL_SUCCESS_TEXT: &str = include_str!("../../../templates/verify_email/email_success.txt");


// Welcome email - sent to new users after account creation
pub const WELCOME_EMAIL_HTML: &str = include_str!("../../../templates/initial_email/welcome_email.html");
pub const WELCOME_EMAIL_TEXT: &str = include_str!("../../../templates/initial_email/welcome_email.txt");


// Invite email - sent to users with an invitation code
pub const INVITE_EMAIL_HTML: &str = include_str!("../../../templates/invite_email/invite_email.html");
pub const INVITE_EMAIL_TEXT: &str = include_str!("../../../templates/invite_email/invite_email.txt");
