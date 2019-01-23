#![allow(dead_code)]

#[cfg(test)]
mod tests {
    #[test]
    fn parse_line() {
        if let super::Line::Author(s) = super::parse_line(&b"--author liyiheng"[..]) {
            assert_eq!("liyiheng".to_string(), s);
        } else {
            panic!("");
        }
    }
}

extern crate chrono;
extern crate ncurses;

pub fn start(mut pages: Vec<Page>) {
    start_with_margin(&mut pages, 3)
}

fn init_ncurses() {
    use ncurses::*;
    initscr();
    raw();
    keypad(ncurses::stdscr(), true);
    noecho();
    start_color();
    setup_bg(COLOR_BLACK);
}

fn setup_bg(c: i16) {
    use ncurses::*;
    init_pair(COLOR_RED, COLOR_RED, c);
    init_pair(COLOR_BLUE, COLOR_BLUE, c);
    init_pair(COLOR_GREEN, COLOR_GREEN, c);
    init_pair(COLOR_WHITE, COLOR_WHITE, c);
    init_pair(COLOR_MAGENTA, COLOR_MAGENTA, c);
    init_pair(COLOR_CYAN, COLOR_CYAN, c);
    init_pair(COLOR_YELLOW, COLOR_YELLOW, c);
}

pub fn start_with_margin(pages: &mut Vec<Page>, margin: u8) {
    let mut state = PageState {
        begin_output: false,
        fg_color: None,
    };
    init_ncurses();
    let l = pages.len();
    let mut i = 0;
    while i < l {
        let p = &pages[i];
        let c = p.show(&mut state, margin);
        ncurses::clear();
        ncurses::mv(0, 0);
        match c {
            'q' => {
                ncurses::endwin();
                return;
            }
            'b' | 'B' => {
                if i > 0 {
                    i -= 1;
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    for p in pages {
        let c = p.show(&mut state, margin);
        match c {
            'q' => {
                ncurses::endwin();
                return;
            }
            'b' | 'B' => {}
            _ => {
                ncurses::clear();
                ncurses::mv(0, 0);
            }
        }
    }
    let _ = ncurses::getch();
    ncurses::endwin();
}

pub struct Page {
    pub lines: Vec<Line>,
    pub title: String,
    pub cur_line: i32,
}
struct PageState {
    pub begin_output: bool,
    pub fg_color: Option<i16>,
}
impl Page {
    fn show(&self, state: &mut PageState, margin: u8) -> char {
        for line in &self.lines {
            let ch = proceed_line(state, line, margin as i32);
            if ch.is_none() {
                continue;
            }
            let ch = ch.unwrap();
            match ch {
                'q' | 'Q' => {
                    return 'q';
                }
                'b' | 'B' => {
                    return 'b';
                }
                _ => {}
            }
        }
        let c = ncurses::getch();
        use std::char;
        return char::from_u32(c as u32).unwrap_or('a');
    }
}
fn show_title(title: &String) {
    let width = ncurses::getmaxx(ncurses::stdscr());
    ncurses::attron(ncurses::A_BOLD());
    let pad = (width - title.len() as i32) / 2;
    if pad > 0 {
        let s = title.clone() + "\n";
        ncurses::mvprintw(3, pad, s.as_str());
    } else {
        // TODO
    }
    ncurses::attroff(ncurses::A_BOLD());
}

fn proceed_line(state: &mut PageState, l: &Line, margin: i32) -> Option<char> {
    match l {
        Line::Comment(_) => None,
        Line::NewPage(_) => None,
        Line::Invalid(_) => None,
        Line::EndOutput => {
            let y = ncurses::getcury(ncurses::stdscr());
            ncurses::mv(y, margin as i32);
            ncurses::addch('`' as ncurses::chtype);
            let mut l = ncurses::getmaxx(ncurses::stdscr()) - margin * 2 - 2;
            while l > 0 {
                ncurses::addch('-' as ncurses::chtype);
                l -= 1;
            }
            ncurses::addch('\'' as ncurses::chtype);
            state.begin_output = false;
            ncurses::mv(y + 1, margin);
            None
        }
        Line::BeginOutput => {
            let y = ncurses::getcury(ncurses::stdscr());
            ncurses::mv(y, margin as i32);
            ncurses::addch('.' as ncurses::chtype);
            let mut l = ncurses::getmaxx(ncurses::stdscr()) - margin * 2 - 2;
            while l > 0 {
                ncurses::addch('-' as ncurses::chtype);
                l -= 1;
            }
            ncurses::addch('.' as ncurses::chtype);
            state.begin_output = true;
            ncurses::mv(y + 1, margin);
            None
        }
        Line::PlainText(v) => {
            let y = ncurses::getcury(ncurses::stdscr());
            let x = ncurses::getcurx(ncurses::stdscr());
            let mut padding = 0;
            if state.begin_output {
                padding = 1;
                ncurses::mv(y, margin);
                ncurses::addch('|' as ncurses::chtype);
                let end = ncurses::getmaxx(ncurses::stdscr()) - margin;
                ncurses::mv(y, end);
                ncurses::addch('|' as ncurses::chtype);
                ncurses::mv(y, x + 1);
            };
            ncurses::mv(y, margin + padding + 1);
            // TODO:
            // 1. Margin top, bottom, left
            // 2. Split line
            ncurses::addstr(v.as_str());
            ncurses::mv(y + 1, margin);
            None
        }
        Line::TripleMinus => {
            let c = ncurses::getch();
            use std::char;
            let c = char::from_u32(c as u32).unwrap_or('a');
            Some(c)
        }
        Line::Author(v) => {
            let y = ncurses::getcury(ncurses::stdscr());
            let x = ncurses::getmaxx(ncurses::stdscr());
            let pad = (x - v.len() as i32) / 2;
            ncurses::mvprintw(y + 1, pad, v.as_str());
            None
        }
        Line::Date(v) => {
            let y = ncurses::getcury(ncurses::stdscr());
            let x = ncurses::getmaxx(ncurses::stdscr());
            if v.len() >= 5 && &v.as_str()[0..5] == "today" {
                let today = chrono::Local::today();
                let format = if v.len() > 6 {
                    &v.as_str()[6..]
                } else {
                    "%b %d %Y"
                };
                let v = today.format(format).to_string();
                let pad = (x - v.len() as i32) / 2;
                ncurses::mvprintw(y + 2, pad, v.as_str());
            } else {
                let pad = (x - v.len() as i32) / 2;
                ncurses::mvprintw(y + 2, pad, v.as_str());
            };
            None
        }
        Line::Title(v) => {
            show_title(v);
            None
        }
        Line::Heading(v) => {
            show_title(v);
            None
        }
        Line::Color(c) => {
            ncurses::attron(ncurses::COLOR_PAIR(*c));
            None
        }
        Line::RevOn => {
            ncurses::attron(ncurses::A_REVERSE());
            None
        }
        Line::RevOff => {
            ncurses::attroff(ncurses::A_REVERSE());
            None
        }
        Line::BoldOn => {
            ncurses::attron(ncurses::A_BOLD());
            None
        }
        Line::BoldOff => {
            ncurses::attroff(ncurses::A_BOLD());
            None
        }
        Line::UnderlineOn => {
            ncurses::attron(ncurses::A_UNDERLINE());
            None
        }
        Line::UnderlineOff => {
            ncurses::attroff(ncurses::A_UNDERLINE());
            None
        }
        Line::BgColor(c) => {
            setup_bg(*c);
            if let Some(fg) = state.fg_color {
                ncurses::bkgd(ncurses::COLOR_PAIR(fg));
            } else {
                ncurses::bkgd(ncurses::COLOR_PAIR(ncurses::COLOR_WHITE));
            }
            None
        }
        Line::FgColor(c) => {
            state.fg_color = Some(*c);
            ncurses::attron(ncurses::COLOR_PAIR(*c));
            None
        }
        Line::Sleep(n) => {
            std::thread::sleep(std::time::Duration::from_secs(*n as u64));
            None
        }
        _ => None,
    }
}

use std::fs::File;
use std::io::BufRead;

pub fn parse_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Page>, std::io::Error> {
    let f = File::open(path)?;
    let lines = std::io::BufReader::new(f).lines();
    let mut pages = vec![];
    pages.push(Page {
        lines: vec![],
        title: String::default(),
        cur_line: 0,
    });

    for (i, l) in lines.enumerate() {
        let _ = l
            .map(|v| {
                let line = parse_line(v.as_ref());
                if let Line::Invalid(_) = line {
                    println!("failed to parse line {}:\n{}", i + 1, v);
                    return;
                }
                if let Line::NewPage(name) = line {
                    let p = Page {
                        title: name,
                        lines: vec![],
                        cur_line: 0,
                    };
                    pages.push(p);
                    return;
                }
                pages.last_mut().unwrap().lines.push(line);
            })
            .map_err(|e| {
                println!("failed to parse line {}, {}", i + 1, e);
            });
    }
    Ok(pages)
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Line {
    Invalid(String),
    PlainText(String), // Common lines
    NewPage(String),   // --newpage [pageName]
    Comment(String),   // --## comments
    Author(String),    // --author liyiheng
    Title(String),     // --title Title for this tpp
    Date(String),      // --date today
    Color(i16),        // --color <color>
    BgColor(i16),      // --bgcolor <color>
    FgColor(i16),      // --fgcolor <color>
    Heading(String),
    HorLine(String),
    Header(String),
    Footer(String),
    Center(String),
    Right(String),
    Exec(String),
    Sleep(i16),
    Huge(String),
    SetHugeFont(String),
    RevOn,
    RevOff,
    UnderlineOn,
    UnderlineOff,
    BeginSlideLeft,
    EndSlideLeft,
    BeginSlideRight,
    EndSlideRight,
    BeginSlideTop,
    EndSlideTop,
    BeginSlideBottom,
    EndSlideBottom,
    WithBorder,
    TripleMinus,
    BeginOutput,
    EndOutput,
    BeginShellOutput,
    EndShellOutput,
    BoldOn,
    BoldOff,
}
fn get_color(s: &String) -> Option<i16> {
    match s.as_str() {
        "red" => Some(ncurses::COLOR_RED),
        "white" => Some(ncurses::COLOR_WHITE),
        "yellow" => Some(ncurses::COLOR_YELLOW),
        "green" => Some(ncurses::COLOR_GREEN),
        "blue" => Some(ncurses::COLOR_BLUE),
        "cyan" => Some(ncurses::COLOR_CYAN),
        "magenta" => Some(ncurses::COLOR_MAGENTA),
        "black" => Some(ncurses::COLOR_BLACK),
        _ => None,
    }
}

pub fn parse_line(dat: &[u8]) -> Line {
    let val_str = |b: &[u8]| -> String { String::from_utf8(b.to_vec()).unwrap_or_default() };
    if dat.len() <= 2 || dat[0] != b'-' || dat[1] != b'-' {
        let s = String::from_utf8(dat.to_vec()).unwrap_or_default();
        return Line::PlainText(s);
    }
    let mut space_i = -1;
    for (i, b) in dat.iter().enumerate() {
        if *b == b' ' {
            space_i = i as i32;
            break;
        }
    }
    let last = if space_i <= 0 {
        dat.len()
    } else {
        space_i as usize
    };
    let value = if space_i >= 0 && (space_i as usize + 1) < dat.len() {
        val_str(&dat[space_i as usize + 1..])
    } else {
        String::default()
    };
    match &dat[2..last] {
        b"author" => Line::Author(value),
        b"newpage" => Line::NewPage(value),
        b"date" => Line::Date(value),
        b"title" => Line::Title(value),
        b"##" => Line::Comment(value),
        b"heading" => Line::Heading(value),
        b"horline" => Line::HorLine(value),
        b"header" => Line::Header(value),
        b"footer" => Line::Footer(value),
        b"color" => get_color(&value)
            .map(|c| Line::Color(c))
            .unwrap_or(Line::Invalid(val_str(dat))),
        b"bgcolor" => get_color(&value)
            .map(|c| Line::BgColor(c))
            .unwrap_or(Line::Invalid(val_str(dat))),
        b"fgcolor" => get_color(&value)
            .map(|c| Line::FgColor(c))
            .unwrap_or(Line::Invalid(val_str(dat))),
        b"center" => Line::Center(value),
        b"right" => Line::Right(value),
        b"exec" => Line::Exec(value),
        b"sleep" => Line::Sleep(value.parse().unwrap_or(3)),
        b"huge" => Line::Huge(value),
        b"sethugefont" => Line::SetHugeFont(value),
        b"-" => Line::TripleMinus,
        b"beginoutput" => Line::BeginOutput,
        b"endoutput" => Line::EndOutput,
        b"beginshelloutput" => Line::BeginShellOutput,
        b"endshelloutput" => Line::EndShellOutput,
        b"boldon" => Line::BoldOn,
        b"boldoff" => Line::BoldOff,
        b"revon" => Line::RevOn,
        b"revoff" => Line::RevOff,
        b"ulon" => Line::UnderlineOn,
        b"uloff" => Line::UnderlineOff,
        b"beginslideleft" => Line::BeginSlideLeft,
        b"endslideleft" => Line::EndSlideLeft,
        b"beginslideright" => Line::BeginSlideRight,
        b"endslideright" => Line::EndSlideRight,
        b"beginslidetop" => Line::BeginSlideTop,
        b"endslidetop" => Line::EndSlideTop,
        b"beginslidebottom" => Line::BeginSlideBottom,
        b"endslidebottom" => Line::EndSlideBottom,
        b"withborder" => Line::WithBorder,
        _ => Line::Invalid(val_str(dat)),
    }
}
