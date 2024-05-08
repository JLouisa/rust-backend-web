use lettre::{SmtpTransport, Transport};

macro_rules! send_email {
    ($email_settings:expr) => {
        async {
            let email_config = crate::utils::constants::get_email_settings();

            let sender_mailbox = format!("This is for me <{}>", $email_settings.host_email)
                .parse()
                .expect("Creating Sender mailbox failed");

            let receiver_mailbox = format!("You <{}>", $email_settings.user_email)
                .parse()
                .expect("Creating Receiver mailbox failed");

            let email = lettre::Message::builder()
                .from(sender_mailbox)
                .to(receiver_mailbox)
                .subject(&$email_settings.subject)
                .header(lettre::message::header::ContentType::TEXT_HTML)
                .body($email_settings.body.clone())
                .expect("Creating email template failed");

            let creds = lettre::transport::smtp::authentication::Credentials::new(
                email_config.email,
                email_config.password,
            );

            let mailer = SmtpTransport::relay(&email_config.host)
                .expect("Creating SMTP Mailer failed")
                .credentials(creds)
                .build();

            match mailer.send(&email) {
                Ok(_) => {
                    println!("Email sent successfully to {}", $email_settings.user_email);
                    Ok(())
                }
                Err(e) => {
                    println!("Could not send email: {:?}", e);
                    Err(e)
                }
            }
        }
    };
}

// Enum to determine the type of email
pub enum EmailType {
    UserVerification,
    PasswordReset,
}

impl EmailType {
    pub async fn send_email(
        &self,
        settings: &EmailSettings,
    ) -> Result<(), lettre::transport::smtp::Error> {
        match self {
            EmailType::PasswordReset => {
                // Use the macro to send a password reset email and properly handle the result
                send_email!(settings).await
            }
            _ => todo!("Email type not implemented"),
        }
    }
}

pub struct EmailSettings {
    pub user_email: String,
    pub host_email: String,
    pub domain: String,
    pub subject: String,
    pub body: String,
}

impl EmailSettings {
    pub fn password_reset_template(
        user_email: String,
        host_email: String,
        domain: String,
        token: String,
    ) -> Self {
        println!("Token: {}", token);
        Self {
            user_email,
            host_email,
            domain: domain.to_string(),
            subject: "Password Reset for RustMX".to_string(),
            body: format!(
                r#"<h1>Password Reset</h1><p>Click <a href='http://{}/reset/{}' target="_blank" rel="noopener noreferrer">here</a> to reset your password</p>"#,
                domain, token
            ),
        }
    }

    pub fn user_verification_reset_template(
        user_email: String,
        email_user: String,
        domain: String,
        token: String,
    ) -> Self {
        Self {
            user_email,
            host_email: email_user,
            domain: domain.to_string(),
            subject: "Verify Your Account".to_string(),
            body: format!(
                r#"<h1>Verify Your Account</h1><p>Click <a href='http://{}/verify/{}' target="_blank" rel="noopener noreferrer">here</a> to verify your account</p>"#,
                domain, token
            ),
        }
    }
}
// "jissicko@gmail.com".to_string()
