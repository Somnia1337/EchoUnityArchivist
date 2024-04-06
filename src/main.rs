use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials};
use lettre::{Message, SmtpTransport, Transport};
use std::io::{self, Write};

fn main() {
    // Read host, username & authorization code
    let host = read_input("SMTP service host: ");
    let username = read_input("username: ");
    let auth_code = read_input("authorization code: ");

    // Open a remote connection to host
    let mailer = SmtpTransport::relay(host.as_str())
        .unwrap()
        .credentials(Credentials::new(username, auth_code))
        .build();

    let email = Message::builder()
        .from(read_input("From: ").parse().unwrap())
        .to(read_input("To: ").parse().unwrap())
        .subject(read_input("Subject: "))
        .header(ContentType::TEXT_PLAIN)
        .body(read_input("Body: ")) // todo: support more lines
        .unwrap();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"), // todo: show the receiver
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_owned()
}
