use crate::domain::datatypes::UserClientForgot;

use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_password_reset_email(
    user_email: &UserClientForgot,
) -> Result<(), lettre::transport::smtp::Error> {
    let email_settings = crate::utils::constants::get_email_settings();
    let email_user = "jissicko@gmail.com".to_string();

    let sender_mailbox: Mailbox = format!("This is for me <{}>", email_settings.email)
        .parse()
        .expect("Creating Sender mailbox failed");

    let receiver_mailbox: Mailbox = format!("You <{}>", email_user)
        .parse()
        .expect("Creating Receiver mailbox failed");

    // Create the email template
    let email = Message::builder()
        .from(sender_mailbox)
        .to(receiver_mailbox)
        .subject("Password Reset for RustMX")
        .header(ContentType::TEXT_HTML)
        .body(format!(
            r#"<h1>Password Reset</h1><p>Click <a href='http://localhost:3000/reset/{}' target="_blank" rel="noopener noreferrer">here</a> to reset your password</p>"#,
            user_email.username
        ))
        .expect("Creating email template failed");

    // Credantials for the email
    let creds = Credentials::new(email_settings.email, email_settings.password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&email_settings.host)
        .expect("Creating SMTP Mailer failed")
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully to {}", user_email.username);
            Ok(())
        }
        Err(e) => {
            println!("Could not send email: {e:?}");
            Err(e)
        }
    }
}
