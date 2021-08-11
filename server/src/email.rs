use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env::var;

/// Tries to send email and returns boolean value indicating whether it's successful or not
pub fn send_email(subject: &str, body: &str, receiver: &str) -> bool {
    let email = Message::builder()
        .from("nerdtree <noreply@nerdtree.org>".parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    let smtp_email = var("SMTP_EMAIL").unwrap(); // assuming that env check is done at startup
    let smtp_password = var("SMTP_PASSWORD").unwrap();
    let smtp_server = var("SMTP_SERVER").unwrap();

    let creds = Credentials::new(smtp_email, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).is_ok()
}
