use echo_unity_archivist::*;

fn main() {
    let mut lang: Option<usize>; // user chosen language

    // Validate user input
    loop {
        let input = read_input(
            "\
> [1] English
  [2] 中文
  Select lang 选择语言: ",
        );
        lang = input.trim().parse().ok();
        match lang {
            Some(1) | Some(2) => break,
            _ => println!("! 1 / 2 !"),
        }
    }

    let prompts = match lang.unwrap() {
        1 => get_prompts(&Language::EN),
        2 => get_prompts(&Language::ZH),
        _ => unreachable!(),
    };

    // Log in to SMTP & IMAP servers
    // todo: allow user to try again in case of failure, instead of `panic!`
    println!("{}", prompts.login);
    let user = User::build(&prompts);

    let smtp_cli = match user.connect_smtp() {
        Ok(transport) => {
            println!("{}{}.", prompts.connected, user.smtp_domain);
            transport
        }
        Err(e) => panic!("{}{:?}", prompts.connecting_smtp_failed, e),
    };

    let mut imap_cli = match user.connect_imap() {
        Ok(session) => {
            println!("{}{}.", prompts.connected, user.imap_domain);
            session
        }
        Err(e) => panic!("{}{:?}", prompts.connecting_imap_failed, e),
    };

    // Perform user actions
    loop {
        let mut act: Option<usize>; // user chosen action

        // Validate user input
        loop {
            let input = read_input(prompts.select_action);
            act = input.trim().parse().ok();
            match act {
                Some(0..=2) => break,
                _ => println!("{}", prompts.invalid_action),
            }
        }

        // Perform action
        // todo: new threads for action handling
        match act.unwrap() {
            0 => break,
            1 => match user.send_email(&smtp_cli, &prompts) {
                Ok(recv) => match recv {
                    None => println!("{}", prompts.sending_canceled),
                    Some(to) => println!("{}{}.", prompts.sent, to),
                },
                Err(e) => println!("{}{:?}", prompts.sending_failed, e),
            },
            2 => match user.fetch_email(&mut imap_cli, &prompts) {
                Ok(option) => match option {
                    None => {}
                    Some(email) => {
                        println!("{}", prompts.message_fetched);
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
                Err(e) => println!("{}{:?}", prompts.reading_message_failed, e),
            },
            _ => unreachable!(), // only to satisfy the compiler
        }
    }

    // Logout & quit
    println!("{}{}...", prompts.logging_out, user.imap_domain);
    imap_cli.logout().unwrap();
    println!("{}", prompts.quitting);
}
