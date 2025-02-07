use crate::myip;
use crate::tui;

pub type Map = std::collections::HashMap<String, String>;

pub fn poll(
    ip_vec: &mut Vec<String>,
    hostname_map: &mut Map,
    up_map: &mut Map,
    mfrm: &mut tui::MsgFrame,
) {
    poll_all(ip_vec, hostname_map, mfrm);
    poll_up(ip_vec, hostname_map, up_map, mfrm);
    ip_vec.sort();
    ip_vec.dedup();

    let mut min_length = 15;
    let mut max_length = 0;
    for ip in ip_vec.clone().iter() {
        if ip.len() < min_length {
            min_length = ip.len();
        }
        if ip.len() > max_length {
            max_length = ip.len();
        }
    }

    let mut ip_vec_by_len: Vec<String> = Vec::new();
    for current_length in min_length..=max_length {
        for ip in ip_vec.clone().iter() {
            if ip.len() == current_length {
                ip_vec_by_len.push(ip.to_string());
            }
        }
    }
    ip_vec.clear();
    for ip in ip_vec_by_len {
        ip_vec.push(ip);
    }
}

// nmap -sn -Pn
pub fn poll_all(ip_vec: &mut Vec<String>, hostname_map: &mut Map, mfrm: &mut tui::MsgFrame) {
    // get my ip address and network address
    let ip_address = myip::get_ip_address();

    let parts: Vec<&str> = ip_address.split('.').collect();
    let mut net_address = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i == 3 {
            net_address.push_str("*");
            break;
        } else {
            net_address.push_str(&part);
        }
        net_address.push_str(".");
    }

    tui::cursor_move(0, 10);
    println!("test 0");

    mfrm.frame.title = "Poll All";
    while mfrm.msg.len() > 0 {
        mfrm.msg.remove(0);
    }
    //tui::push_msg_and_update_frame(mfrm, format!("msg len {}", mfrm.msg.len()));
    //tui::push_msg_and_update_frame(mfrm, format!("msg len {}", mfrm.msg.len()));
    tui::push_msg_and_update_frame(mfrm, format!("nmap -sn -Pn {}", net_address.clone()));

    tui::cursor_move(0, 10);
    println!("test 1");

    let output = std::process::Command::new("nmap")
        .arg("-sn")
        .arg("-Pn")
        .arg(net_address.as_str())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines();
    for line in lines {
        if line.starts_with("Nmap scan") {
            let words: Vec<&str> = line.trim().split(' ').collect();
            if words.len() == 6 {
                let mut ip = words[5].to_string();
                if ip.starts_with('(') {
                    ip.remove(0);
                }
                if ip.ends_with(')') {
                    ip.pop();
                }
                ip_vec.push(ip.clone());
                let _ = match hostname_map.get(&ip.to_string()) {
                    Some(name) => name,
                    None => {
                        hostname_map.insert(ip.to_string(), words[4].to_string().clone());
                        &words[4].to_string()
                    }
                };
            }
        }
    }
}

// nmap -sn
pub fn poll_up(
    ip_vec: &mut Vec<String>,
    hostname_map: &mut Map,
    up_map: &mut Map,
    mfrm: &mut tui::MsgFrame,
) {
    // get my ip address and network address
    let ip_address = myip::get_ip_address();
    let parts: Vec<&str> = ip_address.split('.').collect();
    let mut net_address = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i == 3 {
            net_address.push_str("*");
            break;
        } else {
            net_address.push_str(&part);
        }
        net_address.push_str(".");
    }

    tui::cursor_move(0, 10);
    println!("test 3");

    mfrm.frame.title = "Poll Up";
    while mfrm.msg.len() > 0 {
        mfrm.msg.remove(0);
    }
    //tui::push_msg_and_update_frame(mfrm, format!("msg len {}", mfrm.msg.len()));
    //tui::push_msg_and_update_frame(mfrm, format!("msg len {}", mfrm.msg.len()));
    tui::push_msg_and_update_frame(mfrm, format!("nmap -sn {}", net_address.clone()));

    tui::cursor_move(0, 10);
    println!("test 4");

    let output = std::process::Command::new("nmap")
        .arg("-sn")
        .arg(net_address.as_str())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines = stdout.lines();
    for line in lines {
        if line.starts_with("Nmap scan") {
            let words: Vec<&str> = line.trim().split(' ').collect();

            if words.len() == 5 {
                let ip = words[4];
                ip_vec.push(ip.to_string().clone());
                up_map.insert(ip.to_string(), "up".to_string());
            }

            if words.len() == 6 {
                let mut ip = words[5].to_string();
                if ip.starts_with('(') {
                    ip.remove(0);
                }
                if ip.ends_with(')') {
                    ip.pop();
                }
                ip_vec.push(ip.to_string().clone());
                let _ = match hostname_map.get(&ip.to_string()) {
                    Some(name) => name,
                    None => {
                        hostname_map.insert(ip.to_string(), words[4].to_string().clone());
                        &words[4].to_string()
                    }
                };
                up_map.insert(ip.to_string(), "up".to_string());
            }
        }
    }
}
