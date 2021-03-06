use super::config::*;

use std::env;
use std::io;
use std::process::{Command, Stdio};

#[macro_export]
macro_rules! print_info {
    ($p:literal,$($arg:tt)*) => {{
        print!("{}", format!("[{}] {}\n", $p, format!($($arg)*)));
    }};
}

pub fn file_name_without_extension(f: String) -> String {
    let startwithdot: bool = f.starts_with('.');
    let mut split_by_dot: Vec<&str> = f.split('.').collect();
    split_by_dot.pop();
    let mut returnn = String::new();
    for s in split_by_dot {
        returnn = format!("{} {}", returnn, s);
    }

    if startwithdot { returnn = returnn.trim().to_string(); returnn.insert(0, '.') }

    return returnn.trim().to_string();
}

pub fn escape_string(string: String) -> String {
    return string
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\0", "\0")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");
}

pub fn run_command_with_info(cmd: String, config: Config) -> io::Result<()> {
    if !config.silent {
        print_info!("CMD", "{}", cmd)
    }

    let mut shell_cmd = Command::new("sh");
    shell_cmd.args(["-c", cmd.as_str()]);

    let cwd = format!("{}", env::current_dir()?.display());
    shell_cmd.current_dir(cwd.as_str());

    if !config.piped {
        shell_cmd.stdout(Stdio::inherit());
    } else {
        shell_cmd.stdout(Stdio::null());
    }
    shell_cmd.stderr(Stdio::inherit());

    let status = shell_cmd
        .status()
        .expect(format!("Command {} failed to execute", cmd).as_str());

    match status.code() {
        Some(code) => {
            if code != 0 {
                print_info!("ERROR", "Command exited with `{}` exit code", code);
                std::process::exit(code);
            }
        }
        None => {
            print_info!("ERROR", "Command exited with signal");
            std::process::exit(1);
        }
    }

    Ok(())
}
