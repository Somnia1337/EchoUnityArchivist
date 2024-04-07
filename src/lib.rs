use imap::{self, Session};
use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, Error},
};
use lettre::{Message, SmtpTransport, Transport};
use native_tls::{self, TlsConnector, TlsStream};
use std::{
    io::{self, Write},
    net::TcpStream,
    str,
};

/// Represents an email user.
pub struct User {
    pub smtp_domain: String,
    pub imap_domain: String,
    email: String,
    password: String,
}

impl User {
    /// Constructs a new `User` from user input.
    pub fn build() -> User {
        // Get user input from command line
        let domain = read_input("  Server domain (eg. \"qq.com\" without header): ");
        let email = read_input("  Email: ");
        let password = read_input("  Password (eg. \"jfoaiwnpsej\" SMTP/IMAP password): ");

        User {
            smtp_domain: format!("smtp.{}", domain),
            imap_domain: format!("imap.{}", domain),
            email,
            password,
        }
    }

    /// Connects to the SMTP server with user's credentials.
    ///
    /// # Returns
    ///
    /// A `SmtpTransport` if succeeds, or an `Err` if fails.
    pub fn connect_smtp(&self) -> Result<SmtpTransport, Error> {
        // Open a remote connection to server
        let sender = SmtpTransport::relay(self.smtp_domain.as_str())
            .unwrap()
            .credentials(Credentials::new(self.email.clone(), self.password.clone()))
            .build();

        // Connectivity test & return
        match sender.test_connection() {
            Ok(_) => Ok(sender),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn connect_imap(&self) -> Result<Session<TlsStream<TcpStream>>, imap::error::Error> {
        let domain = self.imap_domain.as_str();
        let tls = TlsConnector::builder().build().unwrap();

        let client = imap::connect((domain, 993), domain, &tls).unwrap();

        match client.login(self.email.clone(), self.password.clone()) {
            Ok(session) => Ok(session),
            Err(e) => Err(e.0),
        }
    }

    /// Sends an email within user input.
    ///
    /// # Returns
    ///
    /// A `String` containing the receiver's email address if succeeds, or an `Error` if fails.
    pub fn send_email(&self, smtp_cli: &SmtpTransport) -> Result<String, Error> {
        println!("> Editing email:");

        // Read & save `to` for returning
        let to = read_input("  To: ");

        // Build the email
        let email = Message::builder()
            .from(self.email.clone().parse().unwrap())
            .to(to.parse().unwrap())
            .subject(read_input("  Subject: "))
            .header(ContentType::TEXT_PLAIN)
            .body(read_body())
            .unwrap();

        // Reconfirm
        let confirmation = read_input(
            "\
> Seems that you've finished editing,
  if everything looks fine,
  enter \"yes\" to confirm sending: ",
        );
        if confirmation.trim().to_lowercase() != "yes" {
            println!("> Sending canceled.");
            return Ok(String::new());
        }

        // Send the email
        println!("> Sending...");
        match smtp_cli.send(&email) {
            Ok(_) => Ok(to),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn read_email(
        &self,
        imap_cli: &mut Session<TlsStream<TcpStream>>,
    ) -> imap::error::Result<Option<String>> {
        let inboxes = imap_cli
            .list(Some(""), Some("*"))?
            .into_iter()
            .filter(|&s| !s.name().starts_with("&"))
            .map(|s| s.name().to_string())
            .collect::<Vec<String>>();
        let size = inboxes.len();

        println!("> Inboxes you can choose from:");
        for (i, inbox) in inboxes.iter().enumerate() {
            println!("  [{}] {}", i + 1, inbox);
        }
        let input = read_input("  Choose an inbox: ");
        let mut inbox: usize = match input.trim().parse().ok() {
            Some(x) => {
                if x >= 1 && x <= size {
                    x
                } else {
                    println!("> Invalid inbox id.");
                    return Ok(None);
                }
            }
            None => {
                println!("> Invalid input.");
                return Ok(None);
            }
        };
        inbox -= 1;
        imap_cli.select(inboxes[inbox].clone())?;
        let messages = imap_cli.fetch("1", "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            println!("> No messages in {}", inboxes[inbox]);
            return Ok(None);
        };

        let body = message.body().expect("message did not have a body!");
        let body = str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        Ok(Some(body))
    }
}

/// Reads user input from command line, with customized prompt.
pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_owned()
}

/// Reads the email's body from user input, until 2 consecutive "Enter"s are met.
pub fn read_body() -> String {
    println!("  Body (press 2 \"Enter\"s in a row to finish):");
    let mut body = String::new();
    io::stdout().flush().expect("failed to flush stdout");

    let mut cnt = 0; // Counter for consecutive empty lines
    loop {
        print!("  ");
        io::stdout().flush().expect("failed to flush stdout");
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
