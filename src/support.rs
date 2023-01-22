use crate::{options::Method, result::Result};
use console::{Key, Term};
use std::{env, io::Write};

const KITTY_TERM: [&str; 1] = ["xterm-kitty"];
const KITTY_PROGRAM: [&str; 1] = ["WezTerm"];
const SIXEL_TERM: [&str; 7] = [
    "xterm-256color",
    "xterm",
    "yaft-256color",
    "st-256color",
    "foot-extra",
    "foot",
    "mlterm",
];
const ITERM_PROGRAM: [&str; 3] = ["iTerm", "WezTerm", "mintty"];
const ITERM_LC: [&str; 3] = ["iTerm", "WezTerm", "mintty"];

fn find_match(list: &[&str], var: &str) -> bool {
    var != "" && list.iter().any(|s| var.contains(s))
}

fn check_attributes(attr_groups: Vec<Vec<&str>>, subcommand: Option<&[u8]>) -> Result<bool> {
    let mut stdout = Term::stdout();
    let command = [subcommand.unwrap_or_default(), b"\x1b[c"].concat();
    stdout.write_all(&command)?;
    stdout.flush()?;

    let mut response = String::new();
    while let Ok(key) = stdout.read_key() {
        if let Key::Char(chr) = key {
            response.push(chr);
            // 'c' should end a primary device attributes response
            if chr == 'c' {
                break;
            }
        }
    }

    // check if each groups of attrs has a match
    Ok(attr_groups.iter().all(|group| find_match(group, &response)))
}

fn kitty() -> bool {
    // term check
    let term = env::var("TERM").unwrap_or_default();
    let program = env::var("TERM_PROGRAM").unwrap_or_default();
    // attrs check
    let attrs = vec![vec!["OK"]];
    let kitty_command = b"\x1b_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1b\\";

    (find_match(&KITTY_TERM, &term) || find_match(&KITTY_PROGRAM, &program))
        && check_attributes(attrs, Some(kitty_command)).unwrap_or(false)
}

fn sixel() -> bool {
    // term check
    let term = env::var("TERM").unwrap_or_default();
    // attrs check
    let attrs = vec![vec![";4;", ";4c"]];

    find_match(&SIXEL_TERM, &term) && check_attributes(attrs, None).unwrap_or(false)
}

fn iterm() -> bool {
    // term check
    let program = env::var("TERM_PROGRAM").unwrap_or_default();
    let lc = env::var("LC_TERMINAL").unwrap_or_default();

    find_match(&ITERM_PROGRAM, &program) || find_match(&ITERM_LC, &lc)
}

pub fn has_support(method: Method) -> bool {
    match method {
        Method::Kitty => kitty(),
        Method::Sixel => sixel(),
        Method::Iterm => iterm(),
        Method::Blocks => true,
    }
}

pub fn truecolor() -> bool {
    let colorterm = env::var("COLORTERM").unwrap_or_default();
    match colorterm.as_str() {
        "truecolor" | "24bit" => true,
        _ => false,
    }
}
