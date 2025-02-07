use crate::nmap;
use crate::ping;
use crate::tui;

use crossterm::style::Color;

pub fn start() {
    let menu_items = vec![("p", "Poll_Network"), ("q", "Quit")];

    loop {
        let selection = tui::menu_horiz(&menu_items);
        match selection {
            'p' => selection_p(),
            'q' => {
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

    let (terminal_width, terminal_height) = tui::tsize();

    let mut mfrm = tui::MsgFrame {
        frame: tui::Frame {
            title: "nmap",
            title_color: Color::Blue,
            frame_color: Color::Green,
            x: 0,
            y: terminal_height - 4,
            w: terminal_width,
            h: 4,
        },
        msg: Vec::new(),
    };

    nmap::poll(&mut ip_vec, &mut hostname_map, &mut up_map, &mut mfrm);

    ping::check_all(&mut ip_vec, &mut up_map, &mut mfrm);

    mfrm.frame.title = "Final Summary";
    tui::push_msg_and_update_frame(&mut mfrm, "".to_string());
    let headers = format!(
        "         {:16} {:20} {:10}\n",
        "ip:", "hostname:", "status:"
    );
    tui::push_msg_and_update_frame(&mut mfrm, headers);
    let headers = format!(
        "         {:16} {:20} {:10}\n",
        "--------------", "------------------", "-------"
    );
    tui::push_msg_and_update_frame(&mut mfrm, headers);
    for (i, ip) in ip_vec.iter().enumerate() {
        let mut buf = String::from("");
        let hostname = match hostname_map.get(ip) {
            Some(name) => name.as_str(),
            None => "",
        };
        let up = match up_map.get(ip) {
            Some(status) => status.as_str(),
            None => "",
        };
        buf.push_str(format!("    {:>3}: {:16} {:20} {:10}", i, ip, hostname, up).as_str());
        tui::push_msg_and_update_frame(&mut mfrm, buf.clone());
    }
    tui::push_msg_and_update_frame(&mut mfrm, "".to_string());
}

fn selection_q() {
    let (_terminal_width, terminal_height) = tui::tsize();
    tui::cursor_move(0, terminal_height);
    tui::show_cursor();
    std::process::exit(0);
}
