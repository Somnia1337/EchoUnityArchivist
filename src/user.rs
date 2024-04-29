use std::{error::Error, str};

use imap::{self, Connection, Session};
use lettre::{
    message::header::ContentType, message::Mailbox, transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};

use crate::*;

/// Represents a user.
pub struct User {
    pub smtp_domain: String,
    pub imap_domain: String,
    pub email_addr: Address,
    password: String,
}

impl User {
    /// Constructs a new `User` from user input.
    pub fn build(prompts: &Prompts) -> User {
        let email = read_email(prompts.login_email_addr, prompts.email_addr_invalid);
        let password = read_input(prompts.login_password);
        let domain = email.domain();

        User {
            smtp_domain: format!("smtp.{}", domain),
            imap_domain: format!("imap.{}", domain),
            email_addr: email,
            password,
        }
    }

    /// Logins to SMTP server with user's credentials.
    ///
    /// # Returns
    ///
    /// An `SmtpTransport` as the SMTP client.
    pub fn login_smtp(&mut self, prompts: &Prompts) -> SmtpTransport {
        loop {
            println!("{}{}...", prompts.login_connecting, self.smtp_domain);
            match self.connect_smtp() {
                Ok(transport) => {
                    println!("{}{}.", prompts.login_connect_succeed, self.smtp_domain);
                    return transport;
                }
                Err(e) => {
                    eprintln!(
                        "{}{}: {:?}",
                        prompts.login_connect_fail,
                        self.smtp_domain,
                        e.source().unwrap()
                    );
                    println!("{}", prompts.login_retry);
                    *self = User::build(prompts);
                }
            }
        }
    }

    /// Logins to IMAP server with user's credentials.
    ///
    /// # Returns
    ///
    /// A `Session<Connection>` as the IMAP client.
    pub fn login_imap(&mut self, prompts: &Prompts) -> Session<Connection> {
        loop {
            println!("{}{}...", prompts.login_connecting, self.imap_domain);
            match self.connect_imap() {
                Ok(session) => {
                    println!("{}{}.", prompts.login_connect_succeed, self.imap_domain);
                    return session;
                }
                Err(e) => {
                    eprintln!(
                        "{}{}: {:?}",
                        prompts.login_connect_fail,
                        self.imap_domain,
                        e.source().unwrap()
                    );
                    println!("{}", prompts.login_retry);
                    *self = User::build(prompts);
                }
            }
        }
    }

    /// Connects to the SMTP server.
    ///
    /// # Returns
    ///
    /// - An `SmtpTransport` if the connection succeeds.
    /// - An `Err` if the connection fails.
    fn connect_smtp(&self) -> Result<SmtpTransport, Box<dyn Error>> {
        // Open a remote connection to server
        let smtp_cli = SmtpTransport::relay(self.smtp_domain.as_str())
            .unwrap()
            .credentials(Credentials::new(
                self.email_addr.to_string(),
                self.password.to_string(),
            ))
            .build();

        // Connectivity test & return
        match smtp_cli.test_connection() {
            Ok(_) => Ok(smtp_cli),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Connects to the IMAP server.
    ///
    /// # Returns
    ///
    /// - A `Session<Connection>` if the connection succeeds.
    /// - An `Err` if the connection fails.
    fn connect_imap(&self) -> imap::error::Result<Session<Connection>> {
        let domain = self.imap_domain.as_str();
        let imap_cli = imap::ClientBuilder::new(domain, 993).connect().unwrap();

        match imap_cli.login(&self.email_addr, &self.password) {
            Ok(session) => Ok(session),
            Err(e) => Err(e.0),
        }
    }

    /// Sends an email within user input.
    ///
    /// # Returns
    ///
    /// - An `Option<String>` if the process succeeds.
    ///     - A `Some` containing the receiver's email address if sending succeeds.
    ///     - A `None` if the user cancels sending during reconfirmation.
    /// - An `Error` if it fails.
    pub fn compose_and_send(
        &self,
        smtp_cli: &SmtpTransport,
        prompts: &Prompts,
    ) -> Result<Option<String>, Box<dyn Error>> {
        println!("{}", prompts.compose_new_message);
        println!("{}", prompts.horizontal_start);

        // Read & save `to` for returning
        let to = read_email(prompts.compose_to, prompts.email_addr_invalid);

        // Build the message
        let email = Message::builder()
            .from(Mailbox::from(self.email_addr.clone()))
            .to(Mailbox::from(to.clone()))
            .subject(read_input(prompts.compose_subject))
            .header(ContentType::TEXT_PLAIN)
            .body(read_body(prompts))
            .unwrap();
        println!("{}", prompts.horizontal_end);
        println!("{}", prompts.compose_editing_finish);

        // Reconfirm
        if !read_reconfirmation(prompts, &RECONFIRMATION) {
            return Ok(None);
        }

        // Send the message
        println!("{}", prompts.send_sending);
        match smtp_cli.send(&email) {
            Ok(_) => Ok(Some(to.to_string())),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Fetches an email from a specific mailbox on the imap server.
    ///
    /// # Returns
    ///
    /// - An `Option<String>` if the process succeeds.
    ///     - A `Some` containing the email's body if an email exists.
    ///     - A `None` if not.
    /// - An `Err` if it fails.
    pub fn fetch_message(
        &self,
        imap_cli: &mut Session<Connection>,
        prompts: &Prompts,
    ) -> imap::error::Result<Option<String>> {
        // Fetch available mailboxes from IMAP server
        println!("{}", prompts.fetch_mailbox);
        let mailboxes = imap_cli
            .list(Some(""), Some("*"))?
            .iter()
            .filter(|&s| !s.name().contains('&'))
            .map(|s| s.name().to_string())
            .collect::<Vec<_>>();
        for (i, mailbox) in mailboxes.iter().enumerate() {
            println!("  [{}] {}", i + 1, mailbox);
        }

        // Select mailbox
        let size = mailboxes.len();
        let mailbox = read_selection(
            prompts.fetch_mailbox_selection,
            prompts.invalid_literal,
            prompts.fetch_mailbox_literal,
            prompts.should_be_one_of_below_literal,
            &RangeUsize { lo: 1, hi: size },
        ) - 1;
        imap_cli.select(&mailboxes[mailbox])?;

        // Fetch all messages in the mailbox and print their "Subject: " line
        let mut i = 1;
        loop {
            let message = imap_cli.fetch(i.to_string(), "RFC822")?;
            if message.is_empty() {
                if i == 1 {
                    println!(
                        "> \"{}\"{}",
                        mailboxes[mailbox], prompts.fetch_mailbox_empty
                    );
                    return Ok(None);
                } else {
                    break;
                }
            }
            if i == 1 {
                println!("{}", prompts.fetch_message_list);
            }
            let subject = message
                .iter()
                .flat_map(|m| {
                    str::from_utf8(m.body().expect("message did not have a body!"))
                        .unwrap()
                        .lines()
                        .map(String::from)
                })
                .find(|l| l.starts_with("Subject:"))
                .map(|s| s[9..].to_string())
                .unwrap();
            println!("  [{}] {}", i, subject);
            i += 1;
        }

        // Fetch the chosen message
        let message = imap_cli.fetch(
            read_selection(
                prompts.fetch_message_selection,
                prompts.invalid_literal,
                prompts.fetch_message_literal,
                prompts.should_be_one_of_below_literal,
                &RangeUsize { lo: 1, hi: i - 1 },
            )
            .to_string(),
            "RFC822",
        )?;
        let message = message.iter().next().unwrap();

        // Parse `Body`
        // todo: support non-ASCII characters
        let body = message.body().expect("message did not have a body!");
        let body = str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        // Return message body
        Ok(Some(body))
    }
}
