use echo_unity_archivist::*;

fn main() {
    // Select lang
    let lang_selection = "\
> 语言 Languages:
  [1] 简体中文
  [2] English
  选择语言 Select a language: ";
    let prompts = match read_selection(lang_selection, "! 必须为 Must be 1 / ", 1, 2) {
        1 => get_prompts(&Lang::ZH),
        2 => get_prompts(&Lang::EN),
        _ => unreachable!(),
    };

    // Welcome
    println!("{}", prompts.eua_welcome);

    // Login to SMTP & IMAP servers and get clients
    println!("{}", prompts.login);
    let mut user = User::build(prompts);
    let smtp_cli = user.login_smtp(prompts);
    let mut imap_cli = user.login_imap(prompts);
    println!("{}{}.", prompts.login_succeed, user.email);

    // Perform user actions
    loop {
        match read_selection(prompts.action_selection, prompts.action_invalid, 0, 2) {
            0 => break,
            1 => match user.compose_and_send(&smtp_cli, prompts) {
                Ok(receiver) => match receiver {
                    None => println!("{}", prompts.send_cancel),
                    Some(to) => println!("{}{}.", prompts.send_succeed, to),
                },
                Err(e) => println!("{}{:?}", prompts.send_fail, e),
            },
            2 => match user.fetch_message(&mut imap_cli, prompts) {
                Ok(message_body) => match message_body {
                    None => {}
                    Some(body) => print_body(body, prompts),
                },
                Err(e) => println!("{}{:?}", prompts.fetch_message_fail, e),
            },
            _ => unreachable!(), // selection from `read_selection` should have matched above
        }
    }

    // Logout from IMAP server
    println!("{}{}...", prompts.eua_logging_out, user.imap_domain);
    match imap_cli.logout() {
        Ok(_) => println!("{}", prompts.eua_logout_succeed),
        Err(e) => println!("{}{:?}", prompts.eua_logout_fail, e),
    }

    // Wait for user to exit
    let _ = read_input(prompts.eua_exit);
}
