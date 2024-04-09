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

pub enum Language {
    EN,
    ZH,
}

pub struct Prompts {
    pub login: &'static str,
    pub domain: &'static str,
    pub email: &'static str,
    pub password: &'static str,
    pub connected: &'static str,
    pub connecting_smtp_failed: &'static str,
    pub connecting_imap_failed: &'static str,
    pub select_action: &'static str,
    pub invalid_action: &'static str,
    pub new_draft: &'static str,
    pub horizontal: &'static str,
    pub to: &'static str,
    pub subject: &'static str,
    pub body: &'static str,
    pub reconfirmation: &'static str,
    pub sending: &'static str,
    pub sent: &'static str,
    pub sending_canceled: &'static str,
    pub sending_failed: &'static str,
    pub fetching_inboxes: &'static str,
    pub select_inbox: &'static str,
    pub invalid_inbox: &'static str,
    pub message_fetched: &'static str,
    pub no_messages: &'static str,
    pub reading_message_failed: &'static str,
    pub logging_out: &'static str,
    pub quitting: &'static str,
}

const PROMPTS_EN: Prompts = Prompts {
    login: "> Logging in is required before interacting with the SMTP/IMAP server.",
    domain: "  Server domain (eg. \"smtp.qq.com\"): ",
    email: "  Email address: ",
    password: "  SMTP/IMAP password (eg. \"jfoaiwnpsej\"): ",
    connected: "> Connected to ",
    connecting_smtp_failed: "! Failed when connecting to SMTP server: ",
    connecting_imap_failed: "! Failed when connecting to IMAP server: ",
    select_action: "\
> Actions:
  [0] Logout & quit
  [1] Send email
  [2] Fetch message
  Select an action: ",
    invalid_action: "! Invalid action: should be 0, 1 or 2.",
    new_draft: "> New draft:",
    horizontal: "  -------------------------------------",
    to: "  To (receiver's email address): ",
    subject: "  Subject: ",
    body: "  Body (press 3 `Enter`s in a row to finish):",
    reconfirmation: "\
> You have finished editing,
  if everything looks fine,
  enter \"yes\" to confirm sending: ",
    sending: "> Sending...",
    sent: "> Your email has been sent to ",
    sending_canceled: "> Sending canceled.",
    sending_failed: "! Sending failed: ",
    fetching_inboxes: "> Fetching inboxes...",
    select_inbox: "  Select an inbox: ",
    invalid_inbox: "! Invalid inbox: should be in between 1 and ",
    message_fetched: "> Message fetched:",
    no_messages: " has no messages.",
    reading_message_failed: "! Could not read email: ",
    logging_out: "> Logging out from ",
    quitting: "> Quitting user agent...",
};

const PROMPTS_ZH: Prompts = Prompts {
    login: "> 在与 SMTP/IMAP 服务器交互之前, 必须登录.",
    domain: "  服务器域名 (如 \"smtp.qq.com\"): ",
    email: "  邮箱: ",
    password: "  SMTP/IMAP 密码 (如 \"jfoaiwnpsej\"): ",
    connected: "> 已连接到 ",
    connecting_smtp_failed: "! 无法连接到 SMTP 服务器: ",
    connecting_imap_failed: "! 无法连接到 IMAP 服务器: ",
    select_action: "\
> 操作:
  [0] 登出 & 关闭
  [1] 发送邮件
  [2] 收取邮件
  选择操作: ",
    invalid_action: "! 无效操作: 应为 0, 1 或 2.",
    new_draft: "> 新草稿:",
    horizontal: "  -------------------------------------",
    to: "  发往 (收件人的邮箱): ",
    subject: "  主题: ",
    body: "  正文 (连按 3 次 `Enter` 键以结束输入):",
    reconfirmation: "\
> 你已完成编辑,
  如果一切无误,
  输入 \"yes\" 以确认发送: ",
    sending: "> 发送中...",
    sent: "> 你的邮件已发往 ",
    sending_canceled: "> 取消发送.",
    sending_failed: "! 发送失败: ",
    fetching_inboxes: "> 获取收件箱...",
    select_inbox: "  选择收件箱: ",
    invalid_inbox: "! 无效收件箱: 应为 1 到 ",
    message_fetched: "> 收到邮件:",
    no_messages: " 没有邮件.",
    reading_message_failed: "! 邮件读取失败: ",
    logging_out: "> 退出登录 ",
    quitting: "> 退出客户代理...",
};

pub fn get_prompts(lang: &Language) -> &'static Prompts {
    match lang {
        Language::EN => &PROMPTS_EN,
        Language::ZH => &PROMPTS_ZH,
    }
}

/// Represents a user.
pub struct User {
    pub smtp_domain: String,
    pub imap_domain: String,
    email: String,
    password: String,
}

impl User {
    /// Constructs a new `User` from user input.
    pub fn build(prompts: &Prompts) -> User {
        let domain = User::sanitize_domain(read_input(prompts.domain));
        let email = read_input(prompts.email);
        let password = read_input(prompts.password);

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
    /// - An `SmtpTransport` if the connection succeeds.
    /// - An `Err` if the connection fails.
    pub fn connect_smtp(&self) -> Result<SmtpTransport, Error> {
        // Open a remote connection to server
        let smtp_cli = SmtpTransport::relay(self.smtp_domain.as_str())
            .unwrap()
            .credentials(Credentials::new(self.email.clone(), self.password.clone()))
            .build();

        // Connectivity test & return
        match smtp_cli.test_connection() {
            Ok(_) => Ok(smtp_cli),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Connects to the IMAP server with the user's credentials.
    ///
    /// # Returns
    ///
    /// - A `Session<TlsStream<TcpStream>>` if the connection succeeds.
    /// - An `Err` if the connection fails.
    pub fn connect_imap(&self) -> imap::error::Result<Session<TlsStream<TcpStream>>> {
        let domain = self.imap_domain.as_str();
        let tls = TlsConnector::builder().build().unwrap();

        let imap_cli = imap::connect((domain, 993), domain, &tls).unwrap();

        match imap_cli.login(self.email.clone(), self.password.clone()) {
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
    pub fn send_email(
        &self,
        smtp_cli: &SmtpTransport,
        prompts: &Prompts,
    ) -> Result<Option<String>, Error> {
        println!("{}", prompts.new_draft);
        println!("{}", prompts.horizontal);

        // Read & save `to` for returning
        let to = read_input(prompts.to);

        // Build the email
        let email = Message::builder()
            .from(self.email.clone().parse().unwrap())
            .to(to.parse().unwrap())
            .subject(read_input(prompts.subject))
            .header(ContentType::TEXT_PLAIN)
            .body(read_body(&prompts))
            .unwrap();
        println!("{}", prompts.horizontal);

        // Reconfirm
        let confirmation = read_input(prompts.reconfirmation);
        if confirmation.trim().to_lowercase() != "yes" {
            return Ok(None);
        }

        // Send the email
        println!("{}", prompts.sending);
        match smtp_cli.send(&email) {
            Ok(_) => Ok(Some(to)),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Fetches an email from a specific inbox on the imap server.
    ///
    /// # Returns
    ///
    /// - An `Option<String>` if the process succeeds.
    ///     - A `Some` containing the email's body if an email exists.
    ///     - A `None` if not.
    /// - An `Err` if it fails.
    pub fn fetch_email(
        &self,
        imap_cli: &mut Session<TlsStream<TcpStream>>,
        prompts: &Prompts,
    ) -> imap::error::Result<Option<String>> {
        // Fetch & show available inboxes from IMAP server
        println!("{}", prompts.fetching_inboxes);
        let inboxes = imap_cli
            .list(Some(""), Some("*"))?
            .into_iter()
            .filter(|&s| !s.name().contains("&"))
            .map(|s| s.name().to_string())
            .collect::<Vec<_>>();
        for (i, inbox) in inboxes.iter().enumerate() {
            println!("  [{}] {}", i + 1, inbox);
        }

        // Select inbox
        let size = inboxes.len();
        let input = read_input(prompts.select_inbox);
        let mut inbox: usize = match input.trim().parse().ok() {
            Some(x) if x >= 1 && x <= size => x,
            _ => {
                println!("{}{}.", prompts.invalid_inbox, size);
                return Ok(None);
            }
        };
        inbox -= 1;
        imap_cli.select(inboxes[inbox].clone())?;

        // Fetch the first message
        // todo: list all available emails to choose from
        let messages = imap_cli.fetch("1", "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            println!("> \"{}\"{}", inboxes[inbox], prompts.no_messages);
            return Ok(None);
        };

        // Parse `Body`
        let body = message.body().expect("message did not have a body!");
        let body = str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        Ok(Some(body))
    }

    /// Sanitizes the domain name from user input, removes prefixing "smtp." or "imap.".
    pub fn sanitize_domain(input: String) -> String {
        if let Some(domain) = input.strip_prefix("smtp.") {
            return domain.to_string();
        } else if let Some(domain) = input.strip_prefix("imap.") {
            return domain.to_string();
        }
        input.to_string()
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

/// Reads the email's body from user input, until 2 consecutive \`Enter\`s are met.
pub fn read_body(prompts: &Prompts) -> String {
    println!("{}", prompts.body);
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
