use crossterm::{cursor, execute, terminal};
use std::env;
use std::io::{Write, stdout};
use std::process::{Command, exit};
use std::{fs, thread, time::Duration};

pub fn watch() -> std::io::Result<()> {
    let mut stdout = stdout();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: fp [path] <command> [arg1] [arg2]");
        exit(1);
    }

    let file_path: &str = match args.get(1) {
        Some(path) => path.as_str(),
        None => {
            println!("Couldn't parse filename!");
            exit(1);
        }
    };

    let list_of_commands: Vec<&str> = args[2..].iter().map(String::as_str).collect();
    let mut last_modified = match fs::metadata(file_path) {
        Ok(last_modified) => match last_modified.modified() {
            Ok(content) => content,
            Err(_) => {
                println!("Unable to read modification time");
                exit(1);
            }
        },
        Err(_) => {
            println!("Unable to read file metadata");
            exit(1);
        }
    };

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let (command, args) = match list_of_commands.as_slice().split_first() {
        Some(pair) => pair,
        None => {
            println!("No commands");
            return Ok(());
        }
    };
    loop {
        let modified = match fs::metadata(file_path) {
            Ok(modified) => match modified.modified() {
                Ok(content) => content,
                Err(_) => {
                    println!("Unable to read latest modification time");
                    exit(1);
                }
            },
            Err(_) => {
                println!("Unable to read file metadata for the most recent modification");
                exit(1);
            }
        };
        if modified != last_modified {
            last_modified = modified;
            execute!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            )?;
            Command::new(command)
                .args(args)
                .status()
                .expect("Failed to execute {command}");
            match stdout.flush() {
                Ok(_) => {}
                Err(_) => {
                    println!("Unable to flush terminal");
                    exit(1);
                }
            };
        }
        thread::sleep(Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(30.5, 30.5);
    }
}
