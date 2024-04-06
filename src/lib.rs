use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, Error},
};
use lettre::{Message, SmtpTransport, Transport};
use std::io::{self, Write};

pub fn connect_smtp_host() -> Result<(SmtpTransport, bool), Error> {
    // Read host, username & authorization code
    let host = read_input("SMTP host: ");
    let username = read_input("username: ");
    let auth_code = read_input("authorization code: ");

    // Open a remote connection to host
    let mailer = SmtpTransport::relay(host.as_str())
        .unwrap()
        .credentials(Credentials::new(username, auth_code))
        .build();

    // Connectivity test & return
    match mailer.test_connection() {
        Ok(b) => Ok((mailer, b)),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn send_email(mailer: SmtpTransport) -> Result<(), Error> {
    let email = Message::builder()
        .from(read_input("From: ").parse().unwrap())
        .to(read_input("To: ").parse().unwrap())
        .subject(read_input("Subject: "))
        .header(ContentType::TEXT_PLAIN)
        .body(read_input("Body: ")) // todo: support more lines
        .unwrap();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_owned()
}
