use echo_unity_archivist::*;

fn main() {
    // Build `User` and login
    println!("You must login before interacting with the server.");
    let user = User::build();
    let sender = match user.connect_smtp() {
        Ok(res) => {
            println!("> Connected to SMTP server.");
            res
        }
        Err(e) => {
            panic!("> Failed connecting to SMTP server: {:?}", e)
        }
    };

    // User can perform several actions before quitting
    loop {
        let mut act: Option<usize>; // user chosen action

        // Validate user input
        loop {
            let input = read_input(
                "\
> Options:
> 0 Quit
> 1 Send an email
Select an option: ",
            );
            act = input.trim().parse().ok();
            match act {
                Some(0) | Some(1) => break,
                _ => println!("> Invalid input: must be an integer in [0,1]"),
            }
        }

        // Perform the action
        match act.unwrap() {
            0 => {
                println!("> Quitting user agent...");
                break;
            }
            1 => {
                match user.send(&sender) {
                    Ok(recv) if !recv.is_empty() => {
                        println!("> Email sent to {} successfully!", recv)
                    }
                    Err(e) => println!("> Could not send email: {:?}", e),
                    _ => {}
                };
            }
            _ => unreachable!(), // `_` to satisfy the compiler
        }
    }
}
