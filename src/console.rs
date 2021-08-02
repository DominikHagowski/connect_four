pub struct Console;
impl Console {
    /// Clears the screen
    pub fn clear() {
        print!("\x1b[2J");   
    }

    /// Sets a console cell's foreground color
    pub fn set_fg_color(id: u8) {
        print!("\x1b[38;5;{}m", id);
    }

    /// Sets a console cell's background color
    pub fn set_bg_color(id: u8) {
        print!("\x1b[48;5;{}m", id);
    }

    /// Resets a console cell's color to the default
    pub fn reset_color() {
        print!("\x1b[0m");
    }
}

pub struct Cursor;
impl Cursor {
    /// Move the cursor to a specified point
    pub fn go_to(x: u8, y: u8) {
        print!("\x1b[{};{}H", y, x);
    }

    /// Move the cursor to the first cell
    pub fn go_home() {
        print!("\x1b[H");
    }
}
