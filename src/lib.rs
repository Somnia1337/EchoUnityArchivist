use imap::{self, Session};
use lettre::{
    message::header::ContentType, message::Mailbox, transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
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
    pub eua_logout_succeed: &'static str,
    pub eua_logout_fail: &'static str,
    pub eua_exit: &'static str,
    pub login: &'static str,
    pub login_email: &'static str,
    pub login_password: &'static str,
    pub login_connecting: &'static str,
    pub login_connect_succeed: &'static str,
    pub login_connect_fail: &'static str,
    pub login_succeed: &'static str,
    pub login_retry: &'static str,
    pub action_selection: &'static str,
    pub action_invalid: &'static str,
    pub compose_new_message: &'static str,
    pub compose_to: &'static str,
    pub compose_subject: &'static str,
    pub compose_content: &'static str,
    pub compose_editing_finish: &'static str,
    pub send_reconfirm: &'static str,
    pub send_reconfirm_invalid: &'static str,
    pub send_sending: &'static str,
    pub send_succeed: &'static str,
    pub send_cancel: &'static str,
    pub send_fail: &'static str,
    pub fetch_mailbox: &'static str,
    pub fetch_mailbox_selection: &'static str,
    pub fetch_mailbox_invalid: &'static str,
    pub fetch_message_succeed: &'static str,
    pub fetch_mailbox_empty: &'static str,
    pub fetch_message_fail: &'static str,
}

/// A `Prompts` constant containing all prompts in Chinese-Simplified.
const PROMPTS_ZH: Prompts = Prompts {
    horizontal: "  -------------------------------------",
    email_invalid: "! 无效邮箱地址: 请检查并重新输入.",
    eua_welcome: "> 谐声收藏家————你的 📧 用户代理.",
    eua_logging_out: "> 正在登出 ",
    eua_logout_succeed: "✓ 已登出.",
    eua_logout_fail: "! 登出失败: ",
    eua_exit: "> 按下 `Enter` 键退出...",
    login: "> 在与 SMTP/IMAP 服务器交互之前, 必须登录.",
    login_email: "  邮箱地址: ",
    login_password: "  SMTP/IMAP 授权码 (不是邮箱密码): ",
    login_connecting: "> 正在连接 ",
    login_connect_succeed: "✓ 已连接到 ",
    login_connect_fail: "! 无法连接 ",
    login_succeed: "> 欢迎回来, ",
    login_retry: "> 重新尝试登录.",
    action_selection: "\
> 操作:
  [0] 登出 & 关闭
  [1] 写信
  [2] 收信
  选择操作: ",
    action_invalid: "! 无效操作: 应为 0, 1 或 ",
    compose_new_message: "> 新邮件:",
    compose_to: "  收件人: ",
    compose_subject: "  主题: ",
    compose_content: "  正文 (连续输入 2 个空行以完成编辑):",
    compose_editing_finish: "> 你已完成编辑.",
    send_reconfirm: "\
> 再次确认:
  [yes] 确认发送
  [no]  取消发送
  确认: ",
    send_reconfirm_invalid: "! 无效确认: 应为 \"yes\" 或 \"no\".",
    send_sending: "> 正在发送...",
    send_succeed: "✓ 你的邮件已发至 ",
    send_cancel: "> 发送已取消.",
    send_fail: "! 发送失败: ",
    fetch_mailbox: "> 正在获取收件箱...",
    fetch_mailbox_selection: "  选择收件箱: ",
    fetch_mailbox_invalid: "! 无效收件箱: 应为 1 到 ",
    fetch_message_succeed: "✓ 收到邮件:",
    fetch_mailbox_empty: " 里没有邮件.",
    fetch_message_fail: "! 读取失败: ",
};

/// A `Prompts` constant containing all prompts in English.
const PROMPTS_EN: Prompts = Prompts {
    horizontal: "  -------------------------------------",
    email_invalid: "! Invalid email, please check and try again.",
    eua_welcome: "> Echo Unity Archivist - your 📧 user agent.",
    eua_logging_out: "> Logging out from ",
    eua_logout_succeed: "✓ Logged out.",
    eua_logout_fail: "! Failed to logout: ",
    eua_exit: "> Press `Enter` to exit...",
    login: "> Login is required before interacting with the SMTP/IMAP server.",
    login_email: "  Email address: ",
    login_password: "  SMTP/IMAP password (not email password): ",
    login_connecting: "> Connecting to ",
    login_connect_succeed: "✓ Connected to ",
    login_connect_fail: "! Failed to connect ",
    login_succeed: "> Welcome back, ",
    login_retry: "> Retry login.",
    action_selection: "\
> Actions:
  [0] Logout & quit
  [1] Compose
  [2] Fetch message
  Select an action: ",
    action_invalid: "! Invalid action: should be 0, 1 or ",
    compose_new_message: "> New message:",
    compose_to: "  To: ",
    compose_subject: "  Subject: ",
    compose_content: "  Content (enter 2 empty lines in a row to finish editing):",
    compose_editing_finish: "> You have finished editing.",
    send_reconfirm: "\
> Reconfirmation:
  [yes] confirm sending
  [no]  cancel
  Confirm: ",
    send_reconfirm_invalid: "! Invalid confirmation: should be \"yes\" or \"no\".",
    send_sending: "> Sending...",
    send_succeed: "✓ Your email has been sent to ",
    send_cancel: "> Sending canceled.",
    send_fail: "! Failed to send message: ",
    fetch_mailbox: "> Fetching mailboxes...",
    fetch_mailbox_selection: "  Select a mailbox: ",
    fetch_mailbox_invalid: "! Invalid mailbox: should be in between 1 and ",
    fetch_message_succeed: "✓ Fetched message:",
    fetch_mailbox_empty: " has no messages.",
    fetch_message_fail: "! Failed to read message: ",
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
    pub email: String,
    password: String,
}

impl User {
    /// Constructs a new `User` from user input.
    pub fn build(prompts: &Prompts) -> User {
        let email = read_email(prompts.login_email, prompts.email_invalid).to_string();
        let password = read_input(prompts.login_password);
        let domain = &email[email.find('@').unwrap() + 1..];

        User {
            smtp_domain: format!("smtp.{}", domain),
            imap_domain: format!("imap.{}", domain),
            email,
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
                    println!("{}{}.", prompts.login_connect_succeed, self.smtp_domain);
                    return transport;
                }
                Err(e) => {
                    println!(
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
    /// Returns a `Session<TlsStream<TcpStream>>` as an IMAP server.
    pub fn login_imap(&mut self, prompts: &Prompts) -> Session<TlsStream<TcpStream>> {
        loop {
            println!("{}{}...", prompts.login_connecting, self.imap_domain);
            match self.connect_imap() {
                Ok(session) => {
                    println!("{}{}.", prompts.login_connect_succeed, self.imap_domain);
                    return session;
                }
                Err(e) => {
                    println!(
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
    pub fn compose_and_send(
        &self,
        smtp_cli: &SmtpTransport,
        prompts: &Prompts,
    ) -> Result<Option<String>, Box<dyn Error>> {
        println!("{}", prompts.compose_new_message);
        println!("{}", prompts.horizontal);

        // Read & save `to` for returning
        let to = read_email(prompts.compose_to, prompts.email_invalid);

        // Build the message
        let email = Message::builder()
            .from(self.email.clone().parse().unwrap())
            .to(to.clone())
            .subject(read_input(prompts.compose_subject))
            .header(ContentType::TEXT_PLAIN)
            .body(read_body(prompts))
            .unwrap();
        println!("{}", prompts.horizontal);
        println!("{}", prompts.compose_editing_finish);

        // Reconfirm
        if !read_reconfirmation(prompts) {
            return Ok(None);
        }

        // Send the message
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
        // Fetch available inboxes from IMAP server
        println!("{}", prompts.fetch_mailbox);
        let inboxes = imap_cli
            .list(Some(""), Some("*"))?
            .into_iter()
            .filter(|&s| !s.name().contains('&'))
            .map(|s| s.name().to_string())
            .collect::<Vec<_>>();
        for (i, inbox) in inboxes.iter().enumerate() {
            println!("  [{}] {}", i + 1, inbox);
        }

        // Select inbox
        let size = inboxes.len();
        let inbox = read_selection(
            prompts.fetch_mailbox_selection,
            prompts.fetch_mailbox_invalid,
            1,
            size,
        ) - 1;
        imap_cli.select(inboxes[inbox].clone())?;

        // Fetch the first message
        // todo: list all available messages to choose from
        let messages = imap_cli.fetch("1", "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            println!("> \"{}\"{}", inboxes[inbox], prompts.fetch_mailbox_empty);
            return Ok(None);
        };

        // Parse `Body`
        // todo: support non-ASCII characters
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
pub fn read_email(prompt_read: &str, prompt_invalid: &str) -> Mailbox {
    loop {
        match read_input(prompt_read).trim().parse().ok() {
            Some(x) => return x,
            _ => println!("{}", prompt_invalid),
        }
    }
}

/// Prompt the user to enter a selection, loops until a valid value is provided.
pub fn read_selection(prompt_read: &str, prompt_invalid: &str, lo: usize, hi: usize) -> usize {
    loop {
        match read_input(prompt_read).trim().parse().ok() {
            Some(x) if x >= lo && x <= hi => return x,
            _ => println!("{}{}.", prompt_invalid, hi),
        }
    }
}

pub fn read_reconfirmation(prompts: &Prompts) -> bool {
    loop {
        let input = read_input(prompts.send_reconfirm).to_lowercase();
        if matches!(input.as_str(), "yes" | "no") {
            return input == "yes";
        } else {
            println!("{}", prompts.send_reconfirm_invalid);
        }
    }
}

/// Reads the email's body from user input, until 2 consecutive empty lines are met.
pub fn read_body(prompts: &Prompts) -> String {
    println!("{}", prompts.compose_content);
    let mut body = String::new();
    let mut buf;

    let mut empty_count = 0;
    while empty_count < 2 {
        buf = read_input("  ") + "\n";
        body += &buf;
        if buf.trim().is_empty() {
            empty_count += 1;
        } else {
            empty_count = 0;
        }
        buf.clear();
    }

    body.trim_end().to_string()
}

/// Prints the real body part of an email, ignores useless headers.
pub fn print_body(email: String, prompts: &Prompts) {
    println!("{}", prompts.fetch_message_succeed);
    println!("{}", prompts.horizontal);
    let mut body = false;
    for line in email.lines() {
        // Real body starts at line "From: "
        if line.starts_with("From: ") {
            body = true;
        }
        // Ignore "Content" & "To" headers
        if body && !(line.starts_with("Content") || line.starts_with("To")) {
            println!("  {}", line);
        }
    }
    println!("{}", prompts.horizontal);
}
