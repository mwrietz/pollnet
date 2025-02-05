#![allow(dead_code)]

use std::env;
use std::io::{self, stdout, Write};

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
};
use getch::Getch;

pub fn menu_horiz(items: &[(&str, &str)]) -> char {
    let (_width, height) = tsize();
    cursor_move(0, height - 1);

    print_prog_name_block();

    for item in items.iter() {
        let buffer = format!("{:>4}", item.0);
        print_color(&buffer, Color::DarkGreen);
        let buffer = format!(":{}", item.1);
        print_color(&buffer, Color::Grey);
    }
    execute!(stdout(), cursor::Hide).unwrap();
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    loop {
        let mut flag = false;
        let g = Getch::new();
        _a = g.getch().unwrap();

        for item in items.iter() {
            let ch = item.0.chars().next().unwrap();
            if (_a as char) == ch {
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }
    }

    _a as char
}

pub fn print_page_header(title: &str) {
    print_title(title, Color::DarkBlue);

    // print version right justified
    let (w, _h) = tsize();
    let prog_name = get_prog_name();
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let offset = prog_name.len() + version.len() + 2;
    cursor_move(w - offset, 1);

    print_color(
        prog_name.as_str(),
        Color::Rgb {
            r: 255,
            g: 135,
            b: 0,
        },
    );
    print_color(" ", Color::Black);
    print_color(
        version.as_str(),
        Color::Rgb {
            r: 255,
            g: 135,
            b: 0,
        },
    );
    println!();
    horiz_line(Color::White);
    cursor_move(0, 4);
}

fn print_prog_name_block() {
    let prog_name = get_prog_name();
    execute!(
        stdout(),
        SetForegroundColor(Color::Black),
        // 208 DarkOrange 255,135,0
        SetBackgroundColor(Color::Rgb {
            r: 255,
            g: 135,
            b: 0
        }),
        Print(format!(" {} ", prog_name)),
        ResetColor
    )
    .expect("print_title_block error");
}

pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn clear_line() {
    execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
}

pub fn cursor_move(x: usize, y: usize) {
    execute!(stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
}

pub fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}

pub fn horiz_line(color: Color) {
    let (width, _) = tsize();
    for _i in 0..width {
        print_color_bold("─", color);
    }
    println!();
}

pub fn pause() {
    let (w, h) = tsize();
    let clear_message = "                            ";
    let message = "Press any key to continue...";
    let message_len: usize = message.len();
    cursor_move((w - message_len) / 2, h - 2);
    print_color(message, Color::DarkBlue);
    std::io::stdout().flush().unwrap();
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
    cursor_move((w - message_len) / 2, h - 2);
    print!("{}", clear_message);
}

pub fn print_color(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str),
        ResetColor
    )
    .expect("print_color error");
}

pub fn print_color_bold(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str.bold()),
        ResetColor
    )
    .expect("print_color_bold error");
}

pub fn print_title(title_string: &str, color: Color) {
    println!();
    for c in title_string.chars() {
        print!(" ");
        print_color_bold(&c.to_string(), color);
    }
    println!();
    horiz_line(color);
    println!();
}

pub fn show_cursor() {
    execute!(stdout(), cursor::Show).unwrap();
}

pub fn splash_screen(line1: &str, line2: &str) {
    cls();
    let (width, height) = tsize();

    let line1_length: usize = line1.len();
    cursor_move(width / 2 - line1_length / 2, height / 2 - 1);
    print_color_bold(line1, Color::DarkBlue);

    let line2_length: usize = line2.len();
    cursor_move(width / 2 - line2_length / 2, height / 2 + 1);
    print_color_bold(
        line2,
        Color::Rgb {
            r: 255,
            g: 135,
            b: 0,
        },
    );

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
    //let one_sec = std::time::Duration::from_millis(1000);
    let dur = std::time::Duration::new(2, 0);
    std::thread::sleep(dur);
    cls();

    execute!(stdout(), cursor::Show).unwrap();
}

//
// TermStat usage:
// let mut termstat = TermStat::default();
//

pub struct TermStat {
    pub line_count: usize,
    pub width: usize,
    pub height: usize,
    pub xpos: usize,
    pub ypos: usize,
}

impl Default for TermStat {
    fn default() -> TermStat {
        let (w, h) = tsize();
        let (x, y) = tpos();
        TermStat {
            line_count: 0,
            width: w,
            height: h,
            xpos: x,
            ypos: y,
        }
    }
}

impl TermStat {
    pub fn line_check(&mut self) {
        let (_x, y) = tpos();
        if y > (self.height - 5) {
            pause();
            cls();
            cursor_move(0, 0);
        }
    }
}

pub fn timestamp() -> String {
    let now = chrono::Local::now();
    now.to_string()
}

pub fn tpos() -> (usize, usize) {
    let pos = crossterm::cursor::position();
    let (x, y) = match pos {
        Ok((x, y)) => (x, y),
        Err(error) => panic!("tpos error: {:?}", error),
    };
    (x as usize, y as usize)
}

pub fn tsize() -> (usize, usize) {
    let size = crossterm::terminal::size();
    let (w, h) = match size {
        Ok((w, h)) => (w, h),
        Err(error) => panic!("tsize error: {:?}", error),
    };
    (w as usize, h as usize)
}

// replaces get_int(), get_float()
pub fn get_val<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let mut buffer = String::new();
        print!("{}", prompt);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        let val: T = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return val;
    }
}

// replaces get_int_default(), get_float_default()
pub fn get_val_default<T: std::str::FromStr + std::fmt::Display>(prompt: &str, default: T) -> T {
    loop {
        let mut buffer = String::new();
        print!("{} [{:.3}]: ", prompt, default);

        std::io::stdout().flush().unwrap();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        if buffer.eq("\n") {
            return default;
        }

        let val: T = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return val;
    }
}

pub fn get_string(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }
    //return buffer;
    buffer
}

pub fn get_string_default(prompt: &str, default: &str) -> String {
    let mut buffer = String::new();
    print!("{} [{}]: ", prompt, default);

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }

    if buffer.eq("") {
        default.to_string()
    } else {
        buffer
    }
}

pub fn dialog_box_get_string(width: usize, height: usize, title: &str, prompt: &str) -> String {
    let (term_width, term_height) = tsize();
    let x = (term_width - width) / 2;
    let y = (term_height - height) / 2;

    let frm = Frame {
        title,
        //title_color: "white",
        title_color: Color::White,
        //frame_color: "white",
        frame_color: Color::White,
        x,
        y,
        w: width,
        h: height,
    };
    frm.display();

    // print title and get string
    cursor_move(x + 2, y);
    print!(" ");
    print_color(title, Color::Red);
    print!(" ");
    cursor_move(x + 3, y + 2);

    get_string(prompt)
}

pub struct Frame<'a> {
    pub title: &'a str,
    pub title_color: Color,
    pub frame_color: Color,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Frame<'_> {
    pub fn clear(&self) {
        // draw middle
        for i in 0..(self.h - 1) {
            cursor_move(self.x + 1, self.y + i + 1);
            for _j in 0..(self.w - 2) {
                print!(" ");
            }
        }
    }
    pub fn display(&self) {
        let ul = "╭";
        let ur = "╮";
        let ll = "╰";
        let lr = "╯";
        let hor = "─";
        let ver = "│";

        // draw top horizontal
        cursor_move(self.x, self.y);
        print_color(ul, self.frame_color);
        for _i in 0..(self.w - 2) {
            print_color(hor, self.frame_color);
        }
        print_color(ur, self.frame_color);

        // draw middle
        for i in 0..(self.h - 1) {
            cursor_move(self.x, self.y + i + 1);
            print_color(ver, self.frame_color);
            for _j in 0..(self.w - 2) {
                print!(" ");
            }
            print_color(ver, self.frame_color);
        }

        // draw bottom horizontal
        cursor_move(self.x, self.y + self.h);
        print_color(ll, self.frame_color);
        for _i in 0..(self.w - 2) {
            print_color(hor, self.frame_color);
        }
        print_color(lr, self.frame_color);
        println!();

        if !self.title.is_empty() {
            cursor_move(self.x + 2, self.y);
            print!(" ");
            print_color(self.title, self.title_color);
            print!(" ");
        }
    }
}

pub struct MsgFrame<'a> {
    pub frame: Frame<'a>,
    pub msg: Vec<&'a str>,
}

impl MsgFrame<'_> {
    pub fn display_msg(&self) {
        for i in 0..self.msg.len() {
            if self.msg.len() > (self.frame.h - 1) {
                if i > (self.msg.len() - self.frame.h) {
                    cursor_move(
                        self.frame.x + 2,
                        self.frame.y + (i - (self.msg.len() - self.frame.h)),
                    );
                    print!("{}", self.msg[i]);
                }
            } else {
                cursor_move(self.frame.x + 2, self.frame.y + (i + 1));
                print!("{}", self.msg[i]);
            }
        }
    }
}
