use std::{io::{stdin, stdout, Write}, process::Command, path::Path, env};

fn main() {
    loop {
        print_current();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Error on Input");

        let mut parts = input.trim().split_whitespace();
        // if parts.count()
        let command;
        if let Some(_command) = parts.next() {
            command = _command;
        } else { continue; }

        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();
                
                match child {
                    Ok(mut child) => { child.wait().unwrap(); },
                    Err(_) => {
                        // eprintln!("epsilon: command not found: {} {:?}", command, args);
                        eprintln!("epsilon: command not found: {}", input);
                    },
                }
            }
        }
    }
}

fn print_current() {
    let current_path = env::current_dir()
                                .unwrap();
    let current_path = current_path.to_str().unwrap();
    print!("{}>", current_path);
    stdout().flush().unwrap();
}