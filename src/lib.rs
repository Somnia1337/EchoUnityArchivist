use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, Error},
};
use lettre::{Message, SmtpTransport, Transport};
use std::io::{self, Write};

/// Represents an email user.
pub struct User {
    server: String,
    email: String,
    password: String,
}

impl User {
    /// Constructs a new `User` from user input.
    pub fn build() -> User {
        // Get user input from command line
        let server = read_input("SMTP server: ");
        let email = read_input("Email: ");
        let password = read_input("SMTP password: ");

        User {
            server,
            email,
            password,
        }
    }

    /// Connects to the server with user's credentials.
    ///
    /// # Returns
    ///
    /// A `SmtpTransport` if succeeds, or an `Err` if fails.
    pub fn connect_smtp(&self) -> Result<SmtpTransport, Error> {
        // Open a remote connection to server
        let sender = SmtpTransport::relay(self.server.clone().as_str())
            .unwrap()
            .credentials(Credentials::new(self.email.clone(), self.password.clone()))
            .build();

        // Connectivity test & return
        match sender.test_connection() {
            Ok(_) => Ok(sender),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Sends an email within user input.
    ///
    /// # Returns
    ///
    /// A `String` containing the receiver's email address if succeeds, or an `Error` if fails.
    pub fn send(&self, sender: &SmtpTransport) -> Result<String, Error> {
        // Read & save `to` for returning
        let to = read_input("To: ");

        // Build the email
        let email = Message::builder()
            .from(self.email.clone().parse().unwrap())
            .to(to.parse().unwrap())
            .subject(read_input("Subject: "))
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
        if confirmation.trim() != "yes" {
            println!("> Sending canceled.");
            return Ok(String::new());
        }

        // Send the email
        match sender.send(&email) {
            Ok(_) => Ok(to),
            Err(e) => Err(Error::from(e)),
        }
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
    print!("Body (press 2 \"Enter\"s in a row to finish): ");
    let mut body = String::new();
    io::stdout().flush().expect("failed to flush stdout");

    let mut cnt = 0; // Counter for consecutive empty lines
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
