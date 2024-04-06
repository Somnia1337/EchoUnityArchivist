use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, Error},
};
use lettre::{Message, SmtpTransport, Transport};
use std::io::{self, Write};

pub struct User {
    host: String,
    username: String,
    auth_code: String,
}

impl User {
    pub fn build() -> User {
        let host = read_input("host: ");
        let username = read_input("username: ");
        let auth_code = read_input("authorization code: ");

        User {
            host,
            username,
            auth_code,
        }
    }

    pub fn connect_smtp(&self) -> Result<(SmtpTransport, bool), Error> {
        let creds = self.get_creds();

        // Open a remote connection to host
        let sender = SmtpTransport::relay(self.get_host().as_str())
            .unwrap()
            .credentials(Credentials::new(creds.0, creds.1))
            .build();

        // Connectivity test & return
        match sender.test_connection() {
            Ok(b) => Ok((sender, b)),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    pub fn get_creds(&self) -> (String, String) {
        (self.username.clone(), self.auth_code.clone())
    }
}

pub fn send(user: &User, sender: &SmtpTransport) -> Result<String, Error> {
    let to = read_input("To: ");

    let email = Message::builder()
        .from(user.get_creds().0.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(read_input("Subject: "))
        .header(ContentType::TEXT_PLAIN)
        .body(read_body())
        .unwrap();

    // Send the email
    match sender.send(&email) {
        Ok(_) => Ok(to),
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

pub fn read_body() -> String {
    print!("Body: ");
    let mut body = String::new();
    io::stdout().flush().expect("failed to flush stdout");

    let mut cnt = 0;
    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("failed to read input");
        body += &buf;
        if buf.trim().is_empty() {
            cnt += 1;
            if cnt == 2 {
                break;
            }
        } else {
            cnt = 0;
        }
    }
    body.trim_end().to_string()
}
