use echo_unity_archivist::*;

fn main() {
    // Select lang
    let lang_selection = "\
> 语言 Languages:
  [1] 简体中文
  [2] English
  设置语言 Set language: ";
    let lang_selection_invalid = "\
! 无效语言 Invalid language: 应为下列值之一 should be one of below
  [1, 2]";
    let prompts = match read_selection(
        lang_selection,
        lang_selection_invalid,
        Selection { lo: 1, hi: 2 },
        false,
    ) {
        1 => get_prompts(&Lang::ZH),
        2 => get_prompts(&Lang::EN),
        _ => unreachable!(),
    };

    // Welcome message
    println!("{}", prompts.eua_welcome);

    // Login to SMTP & IMAP servers to build clients
    println!("{}", prompts.login);
    let mut user = User::build(prompts);
    let smtp_cli = user.login_smtp(prompts);
    let mut imap_cli = user.login_imap(prompts);
    println!("{}{}.", prompts.login_succeed, user.email_addr);

    // Build `Selection` for actions
    let actions = Selection { lo: 0, hi: 2 };

    // Perform user actions
    loop {
        match read_selection(
            prompts.action_selection,
            prompts.action_invalid,
            actions,
            true,
        ) {
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
            _ => unreachable!(), // selection from `read_selection()` should have matched one of the above
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
