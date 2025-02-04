pub fn get_ip_address() -> String {
    let output = std::process::Command::new("ip").arg("a").output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    let lines = stdout.lines();

    //let mut devices: Vec<String> = vec![];
    //let mut dev_buffer = String::from("");

    for line in lines {
        // if line.chars().next().expect("char not found").is_numeric() {
        //     dev_buffer = format!("{:>12} ", line.split(' ').nth(1).expect("nth error"));
        // }
        if line.trim_start().starts_with("inet ") {
            //dev_buffer.push_str(line.trim_start().split(' ').nth(1).expect("nth error"));
            let dev_buffer = line
                .trim_start()
                .split(' ')
                .nth(1)
                .expect("nth error")
                .to_string();
            //addresses.push(dev_buffer.clone());
            let ip = dev_buffer
                .split('/')
                .next()
                .to_owned()
                .expect("ip not found")
                .to_string();
            if ip.starts_with("192") {
                return ip;
            }
        }
    }

    "".to_string()
}
