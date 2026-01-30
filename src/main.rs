use std::fmt;
use std::process::Command;

const BOLD_GREEN_ANSI: &'static str = "\x1b[1;32m";
const BOLD_WHITE_ANSI: &'static str = "\x1b[1;37m";
const NAME_COLUMN_WIDTH: usize = 13;
const WINDOWS_OPEN_COLUMN_WIDTH: usize = 14;

#[derive(Debug)]
struct TmuxSession {
    name: String,
    num_windows: u32,
    date_created: String,
}

impl fmt::Display for TmuxSession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = String::new();
        name.push_str(&self.name);

        let name_padding = NAME_COLUMN_WIDTH - self.name.len() - 1;
        for _ in 0..name_padding {
            name.push(' ');
        }

        let mut num_windows = String::new();
        let win_str = self.num_windows.to_string();
        num_windows.push_str(&win_str);
        let window_padding = WINDOWS_OPEN_COLUMN_WIDTH - win_str.len() - 1;

        for _ in 0..window_padding {
            num_windows.push(' ');
        }

        let line = format!(
            " {}{}|\x1B[0m {}{}|\x1B[0m {}{}\x1B[0m",
            BOLD_WHITE_ANSI, name, BOLD_WHITE_ANSI, num_windows, BOLD_WHITE_ANSI, self.date_created
        );
        write!(f, "{line}")
    }
}

impl TmuxSession {
    fn new(tmux_line: &str) -> TmuxSession {
        // name: n windows (date created)
        let mut name = String::new();
        let mut line_chars = tmux_line.chars();

        while let Some(c) = line_chars.next() {
            if c == ':' {
                break;
            }
            name.push(c);
        }

        // skip the space
        let _ = line_chars.next();
        let mut num_win_str = String::new();

        while let Some(c) = line_chars.next() {
            if c == ' ' {
                break;
            }

            num_win_str.push(c);
        }

        let num_windows: u32 = num_win_str.parse().unwrap();

        let mut date_created = String::new();

        let _ = line_chars.next();

        while let Some(c) = line_chars.next() {
            if c == '(' {
                break;
            }
        }

        while let Some(c) = line_chars.next() {
            if c == ' ' {
                break;
            }
        }

        while let Some(c) = line_chars.next() {
            if c == ')' {
                break;
            }

            date_created.push(c);
        }

        TmuxSession {
            name,
            num_windows,
            date_created,
        }
    }
}

fn print_header() {
    println!(
        " {}Name        \x1B[0m{}|\x1B[0m {}Windows Open \x1B[0m{}|\x1B[0m {}Date Created\x1B[0m",
        BOLD_GREEN_ANSI, BOLD_WHITE_ANSI, BOLD_GREEN_ANSI, BOLD_WHITE_ANSI, BOLD_GREEN_ANSI
    );
    println!(
        "{}---------------------------------------------------------",
        BOLD_WHITE_ANSI
    );
}

fn main() {
    print_header();
    let tmux_output = Command::new("tmux").arg("ls").output().unwrap();
    let std_out = String::from_utf8(tmux_output.stdout).unwrap();

    let tmux_sessions = std_out
        .lines()
        .map(|l| TmuxSession::new(l.as_ref()))
        .collect::<Vec<TmuxSession>>();

    for session in &tmux_sessions {
        println!("{session}")
    }
}
