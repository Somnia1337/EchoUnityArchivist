use echo_unity_archivist::*;

fn main() {
    // Select lang
    let lang_selection = "\
> [1] English
  [2] 简体中文
  Select language 选择语言: ";
    let prompts = match read_selection(lang_selection, "! Must be 必须为 1 / ", 1, 2) {
        1 => get_prompts(&Lang::EN),
        2 => get_prompts(&Lang::ZH),
        _ => unreachable!(),
    };

    // Welcome
    println!("{}", prompts.eua_welcome);

    // Login to SMTP & IMAP servers
    println!("{}", prompts.login);
    let mut user = User::build(&prompts);
    let smtp_cli = user.login_smtp(&prompts);
    let mut imap_cli = user.login_imap(&prompts);

    // Perform user actions
    loop {
        match read_selection(prompts.action_selection, prompts.action_invalid, 0, 2) {
            0 => break,
            1 => match user.send_email(&smtp_cli, &prompts) {
                Ok(recv) => match recv {
                    None => println!("{}", prompts.send_canceled),
                    Some(to) => println!("{}{}.", prompts.send_sent, to),
                },
                Err(e) => println!("{}{:?}", prompts.send_fail, e),
            },
            2 => match user.fetch_message(&mut imap_cli, &prompts) {
                Ok(option) => match option {
                    None => {}
                    Some(email) => print_email_body(email, &prompts),
                },
                Err(e) => println!("{}{:?}", prompts.fetch_message_fail, e),
            },
            _ => unreachable!(), // only to satisfy the compiler
        }
    }

    // Logout & quit
    println!("{}{}...", prompts.eua_logging_out, user.imap_domain);
    imap_cli.logout().unwrap();
    println!("{}", prompts.eua_quitting);
}
