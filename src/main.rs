use echo_unity_archivist::*;

fn main() {
    // Select lang
    let lang_selection = "\
> [1] English
  [2] 简体中文
  Select language 选择语言: ";
    let prompts = match select_usize(lang_selection, "! Must be 必须为 1 / ", 1, 2) {
        1 => get_prompts(&Lang::EN),
        2 => get_prompts(&Lang::ZH),
        _ => unreachable!(),
    };

    // Welcome
    println!("{}", prompts.eua_welcome);

    // Log in to SMTP & IMAP servers
    // todo: allow retry in case of failure
    println!("{}", prompts.login);
    let user = User::build(&prompts);

    let smtp_cli = match user.connect_smtp() {
        Ok(transport) => {
            println!("{}{}.", prompts.login_succeed, user.smtp_domain);
            transport
        }
        Err(e) => panic!("{}{:?}", prompts.login_smtp_fail, e),
    };

    let mut imap_cli = match user.connect_imap() {
        Ok(session) => {
            println!("{}{}.", prompts.login_succeed, user.imap_domain);
            session
        }
        Err(e) => panic!("{}{:?}", prompts.login_imap_fail, e),
    };

    // Perform user actions
    loop {
        // todo: new threads for action handling
        match select_usize(prompts.action_selection, prompts.action_invalid, 0, 2) {
            0 => break,
            1 => match user.send_email(&smtp_cli, &prompts) {
                Ok(recv) => match recv {
                    None => println!("{}", prompts.send_canceled),
                    Some(to) => println!("{}{}.", prompts.send_sent, to),
                },
                Err(e) => println!("{}{:?}", prompts.send_fail, e),
            },
            2 => match user.fetch_email(&mut imap_cli, &prompts) {
                Ok(option) => match option {
                    None => {}
                    Some(email) => {
                        println!("{}", prompts.read_message_fetched);
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
                },
                Err(e) => println!("{}{:?}", prompts.read_message_fail, e),
            },
            _ => unreachable!(), // only to satisfy the compiler
        }
    }

    // Logout & quit
    println!("{}{}...", prompts.eua_logging_out, user.imap_domain);
    imap_cli.logout().unwrap();
    println!("{}", prompts.eua_quitting);
}
