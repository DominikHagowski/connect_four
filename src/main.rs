use std::io;

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

    let mut turn: u8 = 0;

    println!("The game is now starting");

    loop {
        // DRAW
        clear_screen();
        cursor_goto(0, 1);

        // Print the numbers above the board
        // for easier user input
        let numbers = "1234567890";
        print!("{}", &numbers[..(width - 1)]);
        print!("\n");

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

        print!("\n");

        // INPUT

        let current_player = turn % 2;

        println!("Player {}, input x coord between 1 - 9", current_player + 1);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let inputtest = &input[..1];

        let input_num: u8 = (inputtest.parse::<u8>().unwrap()) - 1;

        print!("{}", input_num);

        if (current_player == 0){
            board[input_num as usize][4] = Cell::P2;
        } else {
            board[input_num as usize][4] = Cell::P1;
        }
        

        turn = (turn + 1) % 255;

        // CHECK
    
        //break;
    }

    // EXIT
    print!("\n");
    println!("The game is now over");
}
