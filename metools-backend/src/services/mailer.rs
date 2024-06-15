use lettre::{
    message::header::ContentType,
    transport::smtp::{response::Response, Error},
    Message, SmtpTransport, Transport,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct MailerService {
    smtp_transport: SmtpTransport,
    from_mail: String,
    service_url: String,
}

impl MailerService {
    pub fn init(smtp_transport: SmtpTransport, from_mail: String, service_url: String) -> Self {
        Self {
            smtp_transport,
            from_mail,
            service_url,
        }
    }

    pub fn send(
        &self,
        to_mail: String,
        subject: String,
        content_type: ContentType,
        body: String,
    ) -> Result<Response, Error> {
        let email = Message::builder()
            .from(self.from_mail.parse().unwrap()) // TODO it
            .to(to_mail.parse().unwrap())
            .subject(subject)
            .header(content_type)
            .body(body)
            .unwrap();

        self.smtp_transport.send(&email)
    }

    pub fn send_verification_mail(
        &self,
        to_mail: String,
        verify_key: Uuid,
    ) -> Result<Response, Error> {
        self.send(
            to_mail,
            String::from("Verification"),
            ContentType::TEXT_HTML,
            format!(
                "Your verification link: {}/api/v1/users/verify?verify_key={}&redirect={}",
                self.service_url, verify_key, self.service_url
            ),
        )
    }
}
