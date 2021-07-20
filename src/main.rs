
//set all of the colours used by the program
static COLOR_P1: u8 = 196; //red
static COLOR_P2: u8 = 27; //blue
static COLOR_SEL: u8 = 180; //selected colour

//set all of the dimensions of the board
static BOARD_WIDTH: u8 = 7;
static BOARD_HEIGHT: u8 = 6;

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

fn draw_board() {
    cursor_home();
    
    for y in 1..BOARD_HEIGHT {
        for x in 1..BOARD_WIDTH {
            print!("O");
        }
        print!("\n");
    }

    cursor_home();
}

fn main() {
    clear_screen();

    draw_board();

    cursor_goto(0,20);
}
