
//set all of the colours used by the program
const COLOR_P1: u8 = 196; //red
const COLOR_P2: u8 = 27; //blue
const COLOR_SEL: u8 = 180; //selected colour

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

enum PlayerType {
    Local,
    AI,
    Net,
}

struct Player {
    p_type: PlayerType,
    color: u8,
}

enum GameState {
    In_Progress,
    Finished,
}

struct Result {
    State: GameState,
}

fn draw_board() {
    cursor_home();
    
    for _y in 1..BOARD_HEIGHT {
        for _x in 1..BOARD_WIDTH {
            print!("O");
        }
        print!("\n");
    }

    cursor_home();
}

fn execute_turn(player: &Player) -> Result {
    color_set_foreground(player.color);

    print!("TESTESTESTESTESTEST");

    let Out: Result = Result {
        State: GameState::Finished,
    };

    return Out;
}

fn main() {
    clear_screen();

    draw_board();

    const p1: Player = Player {
        p_type: PlayerType::Local,
        color: COLOR_P1,
    };

    const p2: Player = Player {
        p_type: PlayerType::Local,
        color: COLOR_P2,
    };

    let players: [Player; 2] = [p1, p2];
    let mut turn = 1;

    loop {
        let state: Result = execute_turn(&(players[turn]));
        turn = (turn + 1) % 2;

        match state.State {
            GameState::Finished => {break;},
            GameState::In_Progress => {continue;},
        }

    }

    cursor_goto(0,20);
}
