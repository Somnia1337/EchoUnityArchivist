use echo_unity_archivist::*;

fn main() {
    loop {
        let mut op: Option<usize>;
        loop {
            let input = read_input(
                "\
Options:
0 Quit
1 Send
Select your option: ",
            );
            op = input.trim().parse().ok();
            match op {
                Some(0..=1) => break,
                _ => println!("invalid input: must be a number in [0,1]"),
            }
        }
        match op.unwrap() {
            0 => break,
            1 => {
                // todo: move somewhere else
                let mailer = match connect_smtp_host() {
                    Ok(m) => {
                        if m.1 {
                            println!("Successfully connected."); // todo
                            m.0
                        } else {
                            panic!("unknown connectivity error") // todo
                        }
                    }
                    Err(e) => panic!("{:?}", e), // todo
                };
                match send_email(mailer) {
                    Ok(_) => println!("Email sent successfully!"), // todo: show the receiver
                    Err(e) => println!("Could not send email: {:?}", e),
                };
            }
            // todo: 2 =>
            _ => unreachable!(),
        }
    }
}
