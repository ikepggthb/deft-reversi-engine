extern crate termion;
use std::io::{stdout, Write, stdin};
use termion::*;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

mod board;
use board::*;
type TermOut = AlternateScreen<raw::RawTerminal<std::io::Stdout>>;

fn init_terminal(output: &mut TermOut) -> std::io::Result<()> {
    write!(output, "{}{}", clear::All, cursor::Goto(1, 1))?;
    output.flush()?;
    Ok(())
}

// ----------  title_screen  ----------------

#[derive(Clone)]
enum TitleScreenOption {
    Start,
    Exit,
    None
}

const TITLE: &str = "Deft Reversi";

#[allow(dead_code)]
struct TitleScreenObject {
    name: String,
    x: i32,
    y: i32,
    label : String,
    option: TitleScreenOption
}

fn title_screen(output: &mut TermOut, input: &mut std::io::Stdin) -> std::io::Result<TitleScreenOption> {

    let title_label = TitleScreenObject {
        name: "title label".to_string(),
        x: 1,
        y: 1,
        label: TITLE.to_string(),
        option: TitleScreenOption::None
    };
    let game_start_button = TitleScreenObject {
        name: "start button".to_string(),
        x: 1,
        y: 3,
        label: "Game Start".to_string(),
        option: TitleScreenOption::Start
    };
    let exit_button = TitleScreenObject {
        name: "exit button".to_string(),
        x: 4,
        y: 5,
        label: "Exit".to_string(),
        option: TitleScreenOption::Exit
    };

    let title_object = [&title_label, &game_start_button, &exit_button];
    let mut title_cursor = 0i32;

    print_title_screen(output, &title_object, title_cursor)?;
    for evt in input.events() {
        match evt? {
            // Ctrl-cでプログラム終了
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) =>  {
                return Ok(TitleScreenOption::Exit);
            }
            Event::Key(Key::Char('\r')) | Event::Key(Key::Char('\n')) =>  { // Enter Key
                if let TitleScreenOption::None = title_object[title_cursor as usize].option{
                    continue;
                }
                return Ok(title_object[title_cursor as usize].option.clone());
            }
            Event::Key(Key::Char(key_char)) => {
                match key_char {
                    'w' | 'a' => title_cursor-=1,
                    's' | 'd' => title_cursor+=1,
                    _ => ()
                }
                while title_cursor < 0 {
                    title_cursor += title_object.len() as i32;
                }
                title_cursor = title_cursor % title_object.len() as i32;
                print_title_screen(output, &title_object, title_cursor)?;
            }
            _ => ()
        }
    }

    Ok(TitleScreenOption::Exit)
}

fn print_title_screen(output: &mut TermOut, title_object: &[&TitleScreenObject], title_cursor: i32 )-> std::io::Result<()>{
    init_terminal(output)?;

    for y in 0..8 {
        for x in 0..8 {
            write!(output, " ")?;
            for (i,ob) in title_object.iter().enumerate() {
                if ob.x == x && ob.y == y {
                    write!(output, "{}", ob.label)?;
                    if title_cursor  as usize == i {
                        write!(output, "  <-")?;
                    }
                }
                
            }
        }
        write!(output, "\n")?;
        write!(output, "{}", cursor::Goto(1, y as u16 + 1))?;
    }
    output.flush()?;
    
    Ok(())
}

// ----------  game_board  ----------------

struct BoardCursor {
    x: i32,y: i32
}
impl BoardCursor {
    fn new() -> BoardCursor {BoardCursor { x: 0, y: 0 }}
    fn up(&mut self){
        if self.y == 0 {self.y = 7} else {self.y -= 1};
    }
    fn right(&mut self){
        if self.x == 7 {self.x = 0} else {self.x += 1};
    }
    fn down(&mut self){
        if self.y == 7 {self.y = 0} else {self.y += 1};
    }
    fn left(&mut self){
        if self.x == 0 {self.x = 7} else {self.x -= 1};
    }
}

fn game_screen(output: &mut TermOut, input: &mut std::io::Stdin) -> std::io::Result<()> {
    init_terminal(output)?;
    let mut board = Board::new();
    let mut board_cursor = BoardCursor::new();

    let is_end = |board: &mut Board| -> bool {
        let is_now_pass =  if board.put_able() == 0 {true} else {false}; 
        board.next_turn ^= 1;
        let is_next_pass = if board.put_able() == 0 {true} else {false};
        board.next_turn ^= 1;
        is_now_pass && is_next_pass
    };

    let put = |board: &mut Board, y: i32, x: i32| {
        if is_end(board) {
            eprintln!("End!");
            return;
        }
        if board.put_able() == 0 {
            eprintln!("pass");
            board.next_turn ^= 1;
        }
        if board.next_turn == Board::BLACK {
            //let re_put = board.put_eval_one_simple();
            let re_put = board.put_eval_zero_simple();//board.put_piece_from_coord(y, x);
            if let Err(PutPieceErr::NoValidPlacement) = re_put {
                eprintln!("Err!");
                return;
            }
        } else {
            let re_put;
            if board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() > 48 {
                re_put = board.put_piece(board.end_game_full_solver());
            } else {
                re_put = board.put_eval_one_simple();
            }
            if let Err(PutPieceErr::NoValidPlacement) = re_put {
                eprintln!("Err!");
                return;
            }
        }

    };
    print_board(&board, board_cursor.y, board_cursor.x, output)?;
    for evt in input.events() {
        match evt? {
            // Ctrl-cでプログラム終了
            Event::Key(Key::Ctrl('c')) =>  {
                return Ok(());
            }
            Event::Key(Key::Char(key_char)) => {
                match key_char {
                    'w' => board_cursor.up(),
                    'a' => board_cursor.left(),
                    's' => board_cursor.down(),
                    'd' => board_cursor.right(),
                    'x' => {
                        write!(output, "\nsolveing...\n")?;
                        put(&mut board, board_cursor.y, board_cursor.x);
                    },
                    'p' => board.next_turn ^= 1,
                    'q' => return Ok(()),
                    _ => ()
                }
                print_board(&board, board_cursor.y, board_cursor.x, output)?;
            }
            _ => ()
        }
    }
    Ok(())
}

pub fn print_board(board: &Board, y_now: i32, x_now: i32, output: &mut TermOut) -> std::io::Result<()>{
    init_terminal(output)?;
    write!(output, "black: {}\n", board.bit_board[Board::BLACK].count_ones())?;

    write!(output, "{}", cursor::Goto(1, 2))?;
    write!(output, "white: {}\n", board.bit_board[Board::WHITE].count_ones())?;

    write!(output, "{}", cursor::Goto(1, 3))?;
    for y in 0..8 {
        for x in 0..8 {
            let value: char = {
                if y == y_now && x == x_now {'+'}
                else {
                    let mask: u64 = 1 << y * 8 + x;
                    let put_able_bit = board.put_able();
                    
                    if put_able_bit & mask != 0 {'*'}
                    else if board.bit_board[0] & mask != 0 {'●'}
                    else if board.bit_board[1] & mask != 0 {'○'}
                    else {'.'}
                }
            };
            write!(output, "{} ", value)?;
        }

        write!(output, "\n")?;
        write!(output, "{}", cursor::Goto(1, y as u16 +4))?;
        
    }
    write!(output, "{}", cursor::Goto(1, 13))?;
    write!(output, "{} turn", if board.next_turn == 0 {"Black"} else {"White"})?;

    output.flush()?;
    Ok(())
}


pub fn start() -> std::io::Result<()>{
    
    let mut stdin: std::io::Stdin = stdin();
    let mut output = 
    AlternateScreen::from( stdout().into_raw_mode().unwrap());

    let title_screen_option = title_screen(&mut output, &mut stdin)?;
    match title_screen_option {
        TitleScreenOption::Start => {
            game_screen(&mut output, &mut stdin)?;
        },
        TitleScreenOption::Exit => {
            return Ok(());
        },
        _ => {
            return Ok(());
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    start()?;
    Ok(())
}