
//set all of the colours used by the program
const COLOR_P1: u8 = 196; //red
const COLOR_P2: u8 = 27; //blue

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

#[derive(Copy, Clone)]
enum Cell {
    Unset,
    P1,
    P2,
}

fn main() {

    // SETUP
    clear_screen();
    cursor_home();

    println!("The game is now setting up");
    const width: usize = 7;
    const height: usize = 6;
    let mut board = [[Cell::Unset; width]; height];

    println!("The game is now starting");

    loop {
        // DRAW
   
        for y in 0..width {
            for x in 0..height {

                match &board[x][y] {
                    Cell::Unset => {color_reset();},
                    Cell::P1 => {color_set_foreground(COLOR_P1);},
                    Cell::P2 => {color_set_foreground(COLOR_P2);},
                }

                print!("O");
            }
            print!("\n")
        }

        // INPUT

        // CHECK
    
        break;
    }

    // EXIT
    print!("\n");
    println!("The game is now over");
}
