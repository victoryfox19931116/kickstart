use std::io::{self, Write, BufRead};

use toml;

use errors::{Result, new_error, ErrorKind};


/// Wait for user input and return what they typed
fn read_line() -> Result<String> {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    lines
        .next()
        .and_then(|l| l.ok())
        .ok_or_else(|| new_error(ErrorKind::UnreadableStdin))
}

/// Ask a yes/no question to the user
pub fn ask_bool(prompt: &str, default: bool) -> Result<bool> {
    print!("- {} {}: ", prompt, if default { "[Y/n]" } else { "[y/N]" });
    let _ = io::stdout().flush();
    let input = read_line()?;

    let res = match &*input {
        "y" | "Y" | "yes" | "YES" | "true" => true,
        "n" | "N" | "no" | "NO" | "false" => false,
        "" => default,
        _ => {
            println!("Invalid choice: '{}'", input);
            ask_bool(prompt, default)?
        },
    };

    Ok(res)
}

/// Ask a question to the user where they can write any string
pub fn ask_string(prompt: &str, default: &str) -> Result<String> {
    print!("- {} ({}): ", prompt, default);
    let _ = io::stdout().flush();
    let input = read_line()?;

    let res = match &*input {
        "" => default.to_string(),
        _ => input,
    };

    Ok(res)
}

/// Ask a question to the user where they can write an integer
pub fn ask_integer(prompt: &str, default: i64) -> Result<i64> {
    print!("- {} ({}): ", prompt, default);
    let _ = io::stdout().flush();
    let input = read_line()?;

    let res = match &*input {
        "" => default,
        _ => match input.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid integer: '{}'", input);
                ask_integer(prompt, default)?
            }
        },
    };

    Ok(res)
}

/// Ask users to make a choice between various options
pub fn ask_choices(prompt: &str, default: &toml::Value, choices: &toml::value::Array) -> Result<toml::Value> {
    println!("- {}: ", prompt);
    let mut lines = vec![];
    let mut default_index = 1;

    for (index, choice) in choices.iter().enumerate() {
        println!("{}. {}", index + 1, choice);

        lines.push(format!("{}", index + 1));
        if choice == default {
            default_index = index + 1;
        }
    }

    print!("Choose from {} ({}): ", lines.join(", "), default_index);

    let _ = io::stdout().flush();
    let input = read_line()?;

    let res = match &*input {
        "" => default.clone(),
        _ => {
            if let Ok(num) = input.parse::<usize>() {
                if num > choices.len() {
                    println!("Invalid choice: '{}'", input);
                    ask_choices(prompt, default, choices)?
                } else {
                    choices.get(num - 1).unwrap().clone()
                }
            } else {
                println!("Invalid choice: '{}'", input);
                ask_choices(prompt, default, choices)?
            }
        },
    };

    Ok(res)
}
