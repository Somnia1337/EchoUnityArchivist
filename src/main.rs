use echo_unity_archivist::*;

fn main() {
    // Log in to both SMTP & IMAP servers
    println!("> Logging in is required before interacting with the SMTP/IMAP server.");

    let user = User::build();

    let smtp_cli = match user.connect_smtp() {
        Ok(transport) => {
            println!("> Connected to {}.", user.smtp_domain);
            transport
        }
        Err(e) => panic!("> Failed when connecting to SMTP server: {:?}", e),
    };

    let mut imap_cli = match user.connect_imap() {
        Ok(session) => {
            println!("> Connected to {}.", user.imap_domain);
            session
        }
        Err(e) => panic!("> Failed when connecting to IMAP server: {:?}", e),
    };

    // Perform user actions
    loop {
        let mut act: Option<usize>; // user chosen action

        // Validate user input
        loop {
            let input = read_input(
                "\
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch email
  Select an action: ",
            );
            act = input.trim().parse().ok();
            match act {
                Some(0..=2) => break,
                _ => println!("> Invalid input: should be 0, 1 or 2."),
            }
        }

        // Perform action
        match act.unwrap() {
            0 => break,
            1 => {
                match user.send_email(&smtp_cli) {
                    Ok(recv) if !recv.is_empty() => {
                        println!("> Your email has been sent to {}.", recv)
                    }
                    Err(e) => println!("> Could not send email: {:?}", e),
                    _ => {}
                };
            }
            2 => match user.fetch_email(&mut imap_cli) {
                Ok(option) => match option {
                    None => {}
                    Some(email) => {
                        println!("> Email fetched:");
                        println!("  -------------------------------------");
                        let mut real_body_met = false;
                        for line in email.lines().into_iter() {
                            if line.starts_with("From: ") {
                                real_body_met = true;
                            }
                            if real_body_met && !line.starts_with("Content") {
                                println!("  {}", line);
                            }
                        }
                        println!("  -------------------------------------");
                    }
                },
                Err(e) => println!("> Could not read email: {:?}", e),
            },
            _ => unreachable!(), // only to satisfy the compiler
        }
    }

    // Logout & quit
    println!("> Logging out from {}...", user.imap_domain);
    imap_cli.logout().unwrap();
    println!("> Quitting user agent...");
}
