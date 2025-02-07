use crate::nmap;
use crate::tui;

pub fn check_all(ip_vec: &mut Vec<String>, up_map: &mut nmap::Map, mfrm: &mut tui::MsgFrame) {
    mfrm.frame.title = "Ping Check";
    let mut progress_bar = ".".to_string();

    for ip in ip_vec {
        while mfrm.msg.len() > 0 {
            mfrm.msg.remove(0);
        }

        let msg = format!("{}{}", progress_bar, ip);
        tui::push_msg_and_update_frame(mfrm, msg.clone());

        tui::cursor_move(0, 0);

        //if map doesn't contain status then ping check and log status in hashmap
        if !up_map.contains_key(ip.as_str()) {
            if address_is_up(ip.as_str()) {
                up_map.insert(ip.to_string(), "up".to_string());
            } else {
                up_map.insert(ip.to_string(), "down".to_string());
            }
        }
        // else get status from hashmap
        else {
            let _status = match up_map.get(ip) {
                Some(value) => value,
                None => "",
            };
        }
        progress_bar.push_str(".");
    }
    mfrm.msg = Vec::new();
}

fn address_is_up(ip_str: &str) -> bool {
    let out = std::process::Command::new("ping")
        .arg("-c")
        .arg("3")
        .arg(ip_str)
        .output()
        .expect("Usage: ./ping -c 3 ip.address");

    let out_string = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<&str> = out_string.split('\n').collect();

    for line in lines {
        if line.contains("time=") {
            return true;
        }
        if line.contains("Unreachable") {
            return false;
        }
    }

    false
}
