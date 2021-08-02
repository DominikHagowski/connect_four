pub struct Console;
impl Console {
    pub fn clear() {
        print!("\x1b[2J");   
    }

    pub fn set_fg_color(id: u8) {
        print!("\x1b[38;5;{}m", id);
    }

    pub fn set_bg_color(id: u8) {
        print!("\x1b[48;5;{}m", id);
    }

    pub fn reset_color() {
        print!("\x1b[0m");
    }
}

pub struct Cursor;
impl Cursor {
    pub fn go_to(x: u8, y: u8) {
        print!("\x1b[{};{}H", y, x);
    }

    pub fn go_home() {
        print!("\x1b[H");
    }
}
