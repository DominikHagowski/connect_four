mod console;

use std::io;
use console::{Console, Cursor};

//set all of the colours used by the program
const COLOR_P1: u8 = 196; //red
const COLOR_P2: u8 = 27; //blue

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Unset,
    P1,
    P2,
}

fn main() {
    // SETUP
    Console::clear();
    Cursor::go_home();

    println!("The game is now setting up");
    
    const WIDTH: usize = 7;
    const HEIGHT: usize = 6;
    let mut board = [[Cell::Unset; WIDTH]; HEIGHT];

    let mut turn: u8 = 0;

    println!("The game is now starting");

    loop {
        // DRAW
        Console::clear();
        Cursor::go_to(0, 1);

        // Print the numbers above the board
        // for easier user input
        let numbers = "1234567890";
        print!("{}", &numbers[..(WIDTH - 1)]);
        println!();

        for y in 0..WIDTH {
            for x in 0..HEIGHT {

                match &board[x][y] {
                    Cell::Unset => {Console::reset_color();},
                    Cell::P1 => {Console::set_fg_color(COLOR_P1);},
                    Cell::P2 => {Console::set_fg_color(COLOR_P2);},
                }

                print!("O");
            }
            println!()
        }
        
        Console::reset_color();

        println!();

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
