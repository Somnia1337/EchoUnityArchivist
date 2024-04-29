use crate::read::*;
use crate::types::*;

pub mod read;
pub mod types;
pub mod user;

/// Represents a natural language for CLI.
pub enum Lang {
    EN,
    ZH,
}

/// Represents all prompts for getting user input.
pub struct Prompts {
    pub invalid_literal: &'static str,
    pub should_be_one_of_below_literal: &'static str,
    pub horizontal_start: &'static str,
    pub horizontal_end: &'static str,
    pub email_addr_invalid: &'static str,
    pub eua_welcome: &'static str,
    pub eua_logging_out: &'static str,
    pub eua_logout_succeed: &'static str,
    pub eua_logout_fail: &'static str,
    pub eua_exit: &'static str,
    pub login: &'static str,
    pub login_email_addr: &'static str,
    pub login_password: &'static str,
    pub login_connecting: &'static str,
    pub login_connect_succeed: &'static str,
    pub login_connect_fail: &'static str,
    pub login_succeed: &'static str,
    pub login_retry: &'static str,
    pub action_literal: &'static str,
    pub action_list: &'static str,
    pub action_selection: &'static str,
    pub compose_new_message: &'static str,
    pub compose_to: &'static str,
    pub compose_subject: &'static str,
    pub compose_content: &'static str,
    pub compose_editing_finish: &'static str,
    pub send_confirm_literal: &'static str,
    pub send_reconfirm_list: &'static str,
    pub send_reconfirm_selection: &'static str,
    pub send_sending: &'static str,
    pub send_succeed: &'static str,
    pub send_cancel: &'static str,
    pub send_fail: &'static str,
    pub fetch_mailbox_literal: &'static str,
    pub fetch_mailbox: &'static str,
    pub fetch_mailbox_selection: &'static str,
    pub fetch_mailbox_empty: &'static str,
    pub fetch_message_literal: &'static str,
    pub fetch_message_list: &'static str,
    pub fetch_message_selection: &'static str,
    pub fetch_message_fail: &'static str,
}

/// A `Prompts` constant containing all prompts in Chinese-Simplified.
const PROMPTS_ZH: Prompts = Prompts {
    invalid_literal: "! 无效",
    should_be_one_of_below_literal: "应为下列值之一",
    horizontal_start: "  ----------------邮件开始----------------",
    horizontal_end: "  ----------------邮件结束----------------",
    email_addr_invalid: "! 无效邮箱地址: 请检查并重新输入.",
    eua_welcome: "> 谐声收藏家 0.8.7 - 你的 📧 用户代理.",
    eua_logging_out: "> 正在登出 ",
    eua_logout_succeed: "✓ 已登出.",
    eua_logout_fail: "! 登出失败: ",
    eua_exit: "> 按下 `Enter` 键退出...",
    login: "> 在与 SMTP/IMAP 服务器交互之前, 必须登录.",
    login_email_addr: "  邮箱地址: ",
    login_password: "  SMTP/IMAP 授权码 (不是邮箱密码): ",
    login_connecting: "> 正在连接 ",
    login_connect_succeed: "✓ 已连接到 ",
    login_connect_fail: "! 无法连接 ",
    login_succeed: "> 欢迎回来, ",
    login_retry: "> 重新尝试登录.",
    action_literal: "操作",
    action_list: "\
> 操作:
  [0] 登出 & 关闭
  [1] 写信
  [2] 收信",
    action_selection: "  选择操作: ",
    compose_new_message: "> 新邮件:",
    compose_to: "  收件人: ",
    compose_subject: "  主题: ",
    compose_content: "  正文 (连续输入 2 个空行以完成编辑):",
    compose_editing_finish: "> 你已完成编辑.",
    send_confirm_literal: "确认",
    send_reconfirm_list: "\
> 再次确认:
  [yes] 确认发送
  [no]  取消发送",
    send_reconfirm_selection: "  确认: ",
    send_sending: "> 正在发送...",
    send_succeed: "✓ 你的邮件已发至 ",
    send_cancel: "> 发送已取消.",
    send_fail: "! 发送失败: ",
    fetch_mailbox_literal: "收件箱",
    fetch_mailbox: "> 可选的收件箱:",
    fetch_mailbox_selection: "  选择收件箱: ",
    fetch_mailbox_empty: " 里没有邮件.",
    fetch_message_literal: "邮件",
    fetch_message_list: "✓ 收到邮件:",
    fetch_message_selection: "  选择邮件: ",
    fetch_message_fail: "! 读取失败: ",
};

/// A `Prompts` constant containing all prompts in English.
const PROMPTS_EN: Prompts = Prompts {
    invalid_literal: "! Invalid ",
    should_be_one_of_below_literal: "should be one of below",
    horizontal_start: "  ----------------message starts----------------",
    horizontal_end: "  -----------------message ends-----------------",
    email_addr_invalid: "! Invalid email: please check and try again.",
    eua_welcome: "> Echo Unity Archivist 0.8.7 - your 📧 user agent.",
    eua_logging_out: "> Logging out from ",
    eua_logout_succeed: "✓ Logged out.",
    eua_logout_fail: "! Failed to logout: ",
    eua_exit: "> Press `Enter` to exit...",
    login: "> Login is required before interacting with the SMTP/IMAP server.",
    login_email_addr: "  Email address: ",
    login_password: "  SMTP/IMAP password (not email password): ",
    login_connecting: "> Connecting to ",
    login_connect_succeed: "✓ Connected to ",
    login_connect_fail: "! Failed to connect ",
    login_succeed: "> Welcome back, ",
    login_retry: "> Retry login.",
    action_literal: "action",
    action_list: "\
> Actions:
  [0] Logout & quit
  [1] Compose
  [2] Fetch message",
    action_selection: "  Select an action: ",
    compose_new_message: "> New message:",
    compose_to: "  To: ",
    compose_subject: "  Subject: ",
    compose_content: "  Content (enter 2 empty lines in a row to finish editing):",
    compose_editing_finish: "> You have finished editing.",
    send_confirm_literal: "confirmation",
    send_reconfirm_list: "\
> Reconfirmation:
  [yes] confirm sending
  [no]  cancel",
    send_reconfirm_selection: "  Confirm: ",
    send_sending: "> Sending...",
    send_succeed: "✓ Your email has been sent to ",
    send_cancel: "> Sending canceled.",
    send_fail: "! Failed to send message: ",
    fetch_mailbox_literal: "inbox",
    fetch_mailbox: "> Mailboxes to choose from:",
    fetch_mailbox_selection: "  Select a mailbox: ",
    fetch_mailbox_empty: " has no messages.",
    fetch_message_literal: "message",
    fetch_message_list: "✓ Fetched message:",
    fetch_message_selection: "  Select a message: ",
    fetch_message_fail: "! Failed to read message: ",
};

/// Returns the `Prompts` constant corresponding to the specified `Lang`.
pub fn get_prompts(lang: &Lang) -> &'static Prompts {
    match lang {
        Lang::EN => &PROMPTS_EN,
        Lang::ZH => &PROMPTS_ZH,
    }
}

const RECONFIRMATION: Confirmation = Confirmation::yes_or_no();
