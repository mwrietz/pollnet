use crate::nmap;
use crate::tui;

pub fn check_all(ip_vec: &mut Vec<String>, up_map: &mut nmap::Map) {
    let mut progress_bar = ".".to_string();
    for ip in ip_vec {
        tui::cursor_move(0, 6);
        print!("{}{}", progress_bar, ip);

        //if map doesn't contain status then ping check and log status in hashmap
        if !up_map.contains_key(ip.as_str()) {
            if address_is_up(ip.as_str()) {
                up_map.insert(ip.to_string(), "up".to_string());
                print!("+up");
            } else {
                up_map.insert(ip.to_string(), "down".to_string());
                print!("+dn");
            }
        }
        // else get status from hashmap
        else {
            let status = match up_map.get(ip) {
                Some(value) => value,
                None => "",
            };
            print!("+{}", status);
        }
        //println!();
        progress_bar.push_str(".");
    }
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
