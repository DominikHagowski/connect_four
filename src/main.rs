
//set all of the colours used by the program
static COLOR_P1: u8 = 196; //red
static COLOR_P2: u8 = 27; //blue
static COLOR_SEL: u8 = 180; //selected colour

fn clear_screen() {
    print!("\x1b[2J");   
}

fn cursor_goto(x: u8, y: u8) {
    print!("\x1b[{};{}H", y, x);
}

fn cursor_home() {
    print!("\x1b[H");
}

fn color_set_foreground(id: u8) {
    print!("\x1b[38;5;{}m", id);
}

fn color_set_background(id: u8) {
    print!("\x1b[48;5;{}m", id);
}

fn color_reset() {
    print!("\x1b[0m");
}

fn main() {
    clear_screen();
    println!("Hello, world!");
    
    cursor_goto(11,10);
    print!("Hello i am here now");
    
    cursor_home();
    color_set_foreground(COLOR_P1);
    print!("I am some red text");

    cursor_goto(20, 11);
    color_set_foreground(COLOR_P2);
    print!("I am some blue text :)");

    cursor_goto(3,14);
    color_set_background(COLOR_SEL);
    print!("I am some selected text :P");

    cursor_goto(2,18);
    color_reset();
    print!("I have been stripped of formatting :(");
}
