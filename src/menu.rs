use crate::nmap;
use crate::ping;
use crate::tui;

use crossterm::style::Color;

pub fn start() {
    tui::cls();

    tui::print_page_header("");

    let menu_items = vec![("p", "Poll_Network"), ("q", "Quit")];

    loop {
        //tui::print_page_header("");
        let selection = tui::menu_horiz(&menu_items);
        match selection {
            'p' => selection_p(),
            'q' => {
                tui::cls();
                tui::show_cursor();
                selection_q();
            }
            _ => break,
        }
    }
}

fn selection_p() {
    let mut ip_vec = Vec::new();
    let mut hostname_map = nmap::Map::new();
    let mut up_map = nmap::Map::new();

    tui::cursor_move(0, 4);
    tui::clear_line();
    println!("nmap_poll...");
    nmap::poll(&mut ip_vec, &mut hostname_map, &mut up_map);

    tui::cursor_move(0, 4);
    tui::clear_line();
    println!("ping_check...");
    ping::check_all(&mut ip_vec, &mut up_map);

    tui::cursor_move(0, 4);
    tui::clear_line();
    println!("Final Summary:");
    tui::cursor_move(0, 5);
    tui::clear_line();
    tui::cursor_move(0, 6);
    tui::clear_line();
    let headers = format!(
        "         {:16} {:20} {:10}\n",
        "ip:", "hostname:", "status:"
    );
    tui::print_color(headers.as_str(), Color::Blue);
    let headers = format!(
        "         {:16} {:20} {:10}\n",
        "--------------", "------------------", "-------"
    );
    tui::print_color(headers.as_str(), Color::Blue);
    for (i, ip) in ip_vec.iter().enumerate() {
        let hostname = match hostname_map.get(ip) {
            Some(name) => name.as_str(),
            None => "",
        };
        let up = match up_map.get(ip) {
            Some(status) => status.as_str(),
            None => "",
        };
        if up.contains("up") {
            tui::clear_line();
            let line_num = format!("    {:>3}: ", i);
            tui::print_color(line_num.as_str(), Color::Blue);
            println!("{:16} {:20} {:10}", ip, hostname, up);
        }
    }
}

fn selection_q() {
    std::process::exit(0);
}
