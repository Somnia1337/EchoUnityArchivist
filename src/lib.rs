use imap::{self, Session};
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials};
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};
use native_tls::{self, TlsConnector, TlsStream};
use std::{
    error::Error,
    io::{self, Write},
    net::TcpStream,
    str,
};

/// Represents a natural language for CLI.
pub enum Lang {
    EN,
    ZH,
}

/// Contains all prompts for getting user input.
pub struct Prompts {
    pub horizontal: &'static str,
    pub email_invalid: &'static str,
    pub eua_welcome: &'static str,
    pub eua_logging_out: &'static str,
    pub eua_quitting: &'static str,
    pub login: &'static str,
    pub login_domain: &'static str,
    pub login_email: &'static str,
    pub login_password: &'static str,
    pub login_connecting: &'static str,
    pub login_succeed: &'static str,
    pub login_fail: &'static str,
    pub login_retry: &'static str,
    pub action_selection: &'static str,
    pub action_invalid: &'static str,
    pub send_new_draft: &'static str,
    pub send_to: &'static str,
    pub send_subject: &'static str,
    pub send_body: &'static str,
    pub send_reconfirmation: &'static str,
    pub send_sending: &'static str,
    pub send_sent: &'static str,
    pub send_canceled: &'static str,
    pub send_fail: &'static str,
    pub fetch_inbox: &'static str,
    pub fetch_inbox_selection: &'static str,
    pub fetch_inbox_invalid: &'static str,
    pub fetch_message_fetched: &'static str,
    pub fetch_inbox_empty: &'static str,
    pub fetch_message_fail: &'static str,
}

/// A `Prompts` constant containing all prompts in English.
const PROMPTS_EN: Prompts = Prompts {
    horizontal: "  -------------------------------------",
    email_invalid: "! Invalid email, please check and try again.",
    eua_welcome: "> Echo Unity Archivist - Your 📧 user agent.",
    eua_logging_out: "> Logging out from ",
    eua_quitting: "> Quitting user agent...",
    login: "> Logging in is required before interacting with the SMTP/IMAP server.",
    login_domain: "  Server domain (eg. \"gmail.com\"): ",
    login_email: "  Email address: ",
    login_password: "  SMTP/IMAP password (eg. \"jfoaiwnpsej\"): ",
    login_connecting: "> Connecting to ",
    login_succeed: "> Connected to ",
    login_fail: "! Failed when connecting to ",
    login_retry: "> Retry logging in.",
    action_selection: "\
> Actions:
  [0] Logout & quit
  [1] Send email
  [2] Fetch message
  Select an action: ",
    action_invalid: "! Invalid action: should be 0, 1 or ",
    send_new_draft: "> New draft:",
    send_to: "  To (receiver's email address): ",
    send_subject: "  Subject: ",
    send_body: "  Body (press 3 `Enter`s in a row to finish):",
    send_reconfirmation: "\
> You have finished editing,
  if everything looks fine,
  enter \"yes\" to confirm sending: ",
    send_sending: "> Sending...",
    send_sent: "> Your email has been sent to ",
    send_canceled: "> Sending canceled.",
    send_fail: "! Sending failed: ",
    fetch_inbox: "> Fetching inboxes...",
    fetch_inbox_selection: "  Select an inbox: ",
    fetch_inbox_invalid: "! Invalid inbox: should be in between 1 and ",
    fetch_message_fetched: "> Fetched message:",
    fetch_inbox_empty: " has no messages.",
    fetch_message_fail: "! Could not read email: ",
};

/// A `Prompts` constant containing all prompts in Chinese-Simplified.
const PROMPTS_ZH: Prompts = Prompts {
    horizontal: "  -------------------------------------",
    email_invalid: "! 无效邮箱: 请检查并重新输入.",
    eua_welcome: "> 谐声收藏家————你的 📧 用户代理.",
    eua_logging_out: "> 退出登录 ",
    eua_quitting: "> 退出客户代理...",
    login: "> 在与 SMTP/IMAP 服务器交互之前, 必须登录.",
    login_domain: "  服务器域名 (如 \"qq.com\"): ",
    login_email: "  邮箱: ",
    login_password: "  SMTP/IMAP 密码 (如 \"jfoaiwnpsej\"): ",
    login_connecting: "> 正在连接 ",
    login_succeed: "> 已连接到 ",
    login_fail: "! 无法连接 ",
    login_retry: "> 重新尝试登录.",
    action_selection: "\
> 操作:
  [0] 登出 & 关闭
  [1] 发送邮件
  [2] 收取邮件
  选择操作: ",
    action_invalid: "! 无效操作: 应为 0, 1 或 ",
    send_new_draft: "> 新草稿:",
    send_to: "  发往 (收件人的邮箱): ",
    send_subject: "  主题: ",
    send_body: "  正文 (连按 3 次 `Enter` 键以结束输入):",
    send_reconfirmation: "\
> 你已完成编辑,
  如果一切无误,
  输入 \"yes\" 以确认发送: ",
    send_sending: "> 发送中...",
    send_sent: "> 你的邮件已发往 ",
    send_canceled: "> 取消发送.",
    send_fail: "! 发送失败: ",
    fetch_inbox: "> 获取收件箱...",
    fetch_inbox_selection: "  选择收件箱: ",
    fetch_inbox_invalid: "! 无效收件箱: 应为 1 到 ",
    fetch_message_fetched: "> 收到邮件:",
    fetch_inbox_empty: " 没有邮件.",
    fetch_message_fail: "! 邮件读取失败: ",
};

/// Returns the `Prompts` constant corresponding to the specified `Lang`.
pub fn get_prompts(lang: &Lang) -> &'static Prompts {
    match lang {
        Lang::EN => &PROMPTS_EN,
        Lang::ZH => &PROMPTS_ZH,
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
        let domain = read_input(prompts.login_domain);
        let email = read_email(prompts.login_email, prompts.email_invalid);
        let password = read_input(prompts.login_password);

        User {
            smtp_domain: format!("smtp.{}", domain),
            imap_domain: format!("imap.{}", domain),
            email: email.to_string(),
            password,
        }
    }

    /// Logins to SMTP server with user's credentials.
    ///
    /// Returns an `SmtpTransport` as an SMTP client.
    pub fn login_smtp(&mut self, prompts: &Prompts) -> SmtpTransport {
        loop {
            println!("{}{}...", prompts.login_connecting, self.smtp_domain);
            match self.connect_smtp() {
                Ok(transport) => {
                    println!("{}{}.", prompts.login_succeed, self.smtp_domain);
                    return transport;
                }
                Err(e) => {
                    println!(
                        "{}{}: {:?}",
                        prompts.login_fail,
                        self.smtp_domain,
                        e.source().unwrap()
                    );
                    println!("{}", prompts.login_retry);
                    *self = User::build(&prompts);
                }
            }
        }
    }

    /// Logins to IMAP server with user's credentials.
    ///
    /// Returns a `Session<TlsStream<TcpStream>>` as an IMAP server.
    pub fn login_imap(&mut self, prompts: &Prompts) -> Session<TlsStream<TcpStream>> {
        loop {
            println!("{}{}...", prompts.login_connecting, self.imap_domain);
            match self.connect_imap() {
                Ok(session) => {
                    println!("{}{}.", prompts.login_succeed, self.imap_domain);
                    return session;
                }
                Err(e) => {
                    println!(
                        "{}{}: {:?}",
                        prompts.login_fail,
                        self.imap_domain,
                        e.source().unwrap()
                    );
                    println!("{}", prompts.login_retry);
                    *self = User::build(&prompts);
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
            .credentials(Credentials::new(self.email.clone(), self.password.clone()))
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
    /// - A `Session<TlsStream<TcpStream>>` if the connection succeeds.
    /// - An `Err` if the connection fails.
    fn connect_imap(&self) -> imap::error::Result<Session<TlsStream<TcpStream>>> {
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
    ) -> Result<Option<String>, Box<dyn Error>> {
        println!("{}", prompts.send_new_draft);
        println!("{}", prompts.horizontal);

        // Read & save `to` for returning
        let to = read_email(prompts.send_to, prompts.email_invalid);

        // Build the email
        let email = Message::builder()
            .from(self.email.clone().parse().unwrap())
            .to(to.clone())
            .subject(read_input(prompts.send_subject))
            .header(ContentType::TEXT_PLAIN)
            .body(read_body(&prompts))
            .unwrap();
        println!("{}", prompts.horizontal);

        // Reconfirm
        let confirmation = read_input(prompts.send_reconfirmation);
        if confirmation.trim().to_lowercase() != "yes" {
            return Ok(None);
        }

        // Send the email
        println!("{}", prompts.send_sending);
        match smtp_cli.send(&email) {
            Ok(_) => Ok(Some(to.to_string())),
            Err(e) => Err(Box::new(e)),
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
    pub fn fetch_message(
        &self,
        imap_cli: &mut Session<TlsStream<TcpStream>>,
        prompts: &Prompts,
    ) -> imap::error::Result<Option<String>> {
        // Fetch & show available inboxes from IMAP server
        println!("{}", prompts.fetch_inbox);
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
        let inbox = read_selection(
            prompts.fetch_inbox_selection,
            prompts.fetch_inbox_invalid,
            1,
            size,
        ) - 1;
        imap_cli.select(inboxes[inbox].clone())?;

        // Fetch the first message
        // todo: list all available emails to choose from
        let messages = imap_cli.fetch("1", "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            println!("> \"{}\"{}", inboxes[inbox], prompts.fetch_inbox_empty);
            return Ok(None);
        };

        // Parse `Body`
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

/// Prompt the user to enter an email address, loops until a valid value is provided.
pub fn read_email(read_prompt: &str, invalid_prompt: &str) -> Mailbox {
    let mut selection: Option<Mailbox>;

    loop {
        let input = read_input(read_prompt);
        selection = input.trim().parse().ok();
        match selection {
            Some(x) => return x,
            _ => println!("{}", invalid_prompt),
        }
    }
}

/// Prompt the user to enter a selection, loops until a valid value is provided.
pub fn read_selection(read_prompt: &str, invalid_prompt: &str, lo: usize, hi: usize) -> usize {
    let mut selection: Option<usize>;

    loop {
        let input = read_input(read_prompt);
        selection = input.trim().parse().ok();
        match selection {
            Some(x) if x >= lo && x <= hi => return x,
            _ => println!("{}{}.", invalid_prompt, hi),
        }
    }
}

/// Reads the email's body from user input, until 2 consecutive empty lines are met.
pub fn read_body(prompts: &Prompts) -> String {
    println!("{}", prompts.send_body);
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

pub fn print_email_body(email: String, prompts: &Prompts) {
    println!("{}", prompts.fetch_message_fetched);
    println!("{}", prompts.horizontal);
    let mut real_body_met = false;
    for line in email.lines().into_iter() {
        if line.starts_with("From: ") {
            real_body_met = true;
        }
        if real_body_met && !line.starts_with("Content") {
            println!("  {}", line);
        }
    }
    println!("{}", prompts.horizontal);
}
