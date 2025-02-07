mod menu;
mod myip;
mod nmap;
mod ping;
mod tui;

fn main() {
    // tui::splash_screen(
    //     &tui::get_prog_name(),
    //     format!("v{}", env!("CARGO_PKG_VERSION")).as_str(),
    // );

    menu::start();
}
