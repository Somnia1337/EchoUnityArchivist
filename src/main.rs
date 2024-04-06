use echo_unity_archivist::*;

fn main() {
    let user = User::build();
    let sender = match user.connect_smtp() {
        Ok(res) => {
            if res.1 {
                println!("Connected to SMTP host.");
                res.0
            } else {
                panic!("Failed connecting to SMTP host: connection closed")
            }
        }
        Err(e) => {
            panic!("Failed connecting to SMTP host: {:?}", e)
        }
    };

    loop {
        let mut op: Option<usize>;

        loop {
            let input = read_input(
                "\
Options:
0 Quit
1 Send an email
Select an option: ",
            );
            op = input.trim().parse().ok();
            match op {
                Some(0) | Some(1) => break,
                _ => println!("invalid input: must be an integer in [0,1]"),
            }
        }

        match op.unwrap() {
            0 => break,
            1 => {
                match send(&user, &sender) {
                    Ok(recv) => println!("Email sent to {} successfully!", recv),
                    Err(e) => println!("Could not send email: {:?}", e),
                };
            }
            _ => unreachable!(),
        }
    }
}
