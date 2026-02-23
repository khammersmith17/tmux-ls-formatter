use std::fmt;
use std::process::Command;
use std::sync::OnceLock;

const BOLD_GREEN_ANSI: &'static str = "\x1b[1;32m";
const BOLD_WHITE_ANSI: &'static str = "\x1b[1;37m";
const DEFAULT_NAME_COLUMN_WIDTH: usize = 6;
const WINDOWS_OPEN_COLUMN_WIDTH: usize = 12;
const DEFAULT_TS_COLUMN_WIDTH: usize = 12;
const HEADER_SEP_PADDING: usize = 6;

const NAME_HEADER: &'static str = "Name";
const WINDOW_HEADER: &'static str = "Windows Open";
const TS_HEADER: &'static str = "Date Created";

static NAME_COLUMN_WIDTH: OnceLock<usize> = OnceLock::new();
static HEADER_SEP_WIDTH: OnceLock<usize> = OnceLock::new();

#[derive(Debug)]
struct TmuxSession {
    name: String,
    num_windows: u32,
    date_created: String,
}

fn set_column_widths(sessions: &[TmuxSession]) {
    let (mut name_width, mut ts_width) = if sessions.is_empty() {
        (DEFAULT_NAME_COLUMN_WIDTH, DEFAULT_TS_COLUMN_WIDTH)
    } else {
        let mut max_session_name_len = 0_usize;
        let mut max_ts_len = 0_usize;

        for session in sessions {
            max_session_name_len = usize::max(max_session_name_len, session.name.len());
            max_ts_len = usize::max(max_ts_len, session.date_created.len());
        }
        (max_session_name_len, max_ts_len)
    };

    name_width = usize::max(name_width + 1, DEFAULT_NAME_COLUMN_WIDTH);
    ts_width = usize::max(ts_width, DEFAULT_TS_COLUMN_WIDTH);

    NAME_COLUMN_WIDTH.set(name_width).unwrap();
    HEADER_SEP_WIDTH
        .set(name_width + WINDOWS_OPEN_COLUMN_WIDTH + ts_width + HEADER_SEP_PADDING)
        .unwrap();
}

impl fmt::Display for TmuxSession {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = format!(
            " {}{:<name_col_width$}|\x1B[0m {}{:<nw_col_width$}|\x1B[0m {}{}\x1B[0m",
            BOLD_WHITE_ANSI,
            self.name,
            BOLD_WHITE_ANSI,
            self.num_windows,
            BOLD_WHITE_ANSI,
            self.date_created,
            name_col_width = NAME_COLUMN_WIDTH.get().unwrap(),
            nw_col_width = WINDOWS_OPEN_COLUMN_WIDTH + 1
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
        " {}{:<name_col_width$}\x1B[0m{}|\x1B[0m {}{:<window_col_width$} \x1B[0m{}|\x1B[0m {}{:<ts_col_width$}\x1B[0m",
        BOLD_GREEN_ANSI,
        NAME_HEADER,
        BOLD_WHITE_ANSI,
        BOLD_GREEN_ANSI,
        WINDOW_HEADER,
        BOLD_WHITE_ANSI,
        BOLD_GREEN_ANSI,
        TS_HEADER,
        name_col_width = NAME_COLUMN_WIDTH.get().unwrap(),
        window_col_width = WINDOWS_OPEN_COLUMN_WIDTH,
        ts_col_width = DEFAULT_TS_COLUMN_WIDTH
    );
    println!(
        "{}{}",
        BOLD_WHITE_ANSI,
        "-".repeat(*HEADER_SEP_WIDTH.get().unwrap())
    );
}

fn main() {
    let tmux_output = Command::new("tmux").arg("ls").output().unwrap();
    let std_out = String::from_utf8(tmux_output.stdout).unwrap();

    let tmux_sessions = std_out
        .lines()
        .map(|l| TmuxSession::new(l.as_ref()))
        .collect::<Vec<TmuxSession>>();

    set_column_widths(&tmux_sessions);
    print_header();

    for session in &tmux_sessions {
        println!("{session}")
    }
}
