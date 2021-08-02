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

#[derive(Copy, Clone, PartialEq)]
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
    
    const WIDTH: usize = 7;
    const HEIGHT: usize = 6;
    let mut board = [[Cell::Unset; WIDTH]; HEIGHT];

    let mut turn: u8 = 0;

    println!("The game is now starting");

    loop {
        // DRAW
        clear_screen();
        cursor_goto(0, 1);

        // Print the numbers above the board
        // for easier user input
        let numbers = "1234567890";
        print!("{}", &numbers[..(WIDTH - 1)]);
        println!();

        for y in 0..WIDTH {
            for x in 0..HEIGHT {

                match &board[x][y] {
                    Cell::Unset => {color_reset();},
                    Cell::P1 => {color_set_foreground(COLOR_P1);},
                    Cell::P2 => {color_set_foreground(COLOR_P2);},
                }

                print!("O");
            }
            print!("\n")
        }
        
        color_reset();

        print!("\n");

        // INPUT
        loop {
            let current_player = turn % 2;

            println!("Player {}, input x coord,", current_player + 1);
            println!("numbers above the WIDTH will be wrapped around");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            let inputtest = &input[..1];

            let input_num: u8 = ((inputtest.parse::<u8>().unwrap())) - 1;

            print!("{}", input_num);

            let mut board_min = HEIGHT;

            loop {

                if Cell::Unset == board[input_num as usize][board_min] {
                    break;
                }

                if board_min == 0 {
                    break;
                }
                board_min = board_min - 1;
            }

            if current_player == 0{
                board[input_num as usize][board_min] = Cell::P2;
            } else {
                board[input_num as usize][board_min] = Cell::P1;
            }

            break;
        }

        turn = (turn + 1) % 255;

        // CHECK

        let mut Winner = Cell::Unset;

        Winner = Cell::Unset;

        for y in 0..(HEIGHT - 1) {
            for x in 0..(WIDTH - 1) {
                let current_player = &board[x][y];

                if &Cell::Unset == current_player {
                    continue;
                }

                if x + 3 < WIDTH && 
                    &board[x + 1][y] == current_player &&
                    &board[x + 2][y] == current_player &&
                    &board[x + 3][y] == current_player {
                    Winner = Cell::P1;
                }

                if y + 3 < HEIGHT && 
                    &board[x][y + 1] == current_player &&
                    &board[x][y + 2] == current_player &&
                    &board[x][y + 3] == current_player {
                    Winner = Cell::P2;
                }
            }
        }

        match Winner {
            Cell::Unset => {continue;},
            Cell::P1 => {println!("\nWinner: player 1"); break;},
            Cell::P2 => {println!("\nWinner: player 2"); break;},
        }
    }

    // EXIT
    println!();
    println!("The game is now over");
}
