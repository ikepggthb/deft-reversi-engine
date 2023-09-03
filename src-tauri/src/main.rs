// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Menu, CustomMenuItem, Submenu, WindowMenuEvent, Wry, Manager,};

use tauri::State;
mod board;
use board::*;
mod ai;
use ai::*;

const BLACK: i32 = 1;
const WHITE: i32 = 2;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

pub struct BoardManager {
    board_list: Mutex<Vec<Board>>
}

impl BoardManager {
    pub fn new() -> Self {
        Self { board_list: Mutex::new(Vec::new()) }
    }
    pub fn current_board(&self) -> Board {
        let mut board_list = self.board_list.lock().unwrap();
        board_list.last_mut().unwrap().clone()
    }

    pub fn undo(&self) -> Board {
        let mut board_list = self.board_list.lock().unwrap();
        board_list.pop().unwrap()
    }
    pub fn add(&self, board: Board){
        let mut board_list = self.board_list.lock().unwrap();
        board_list.push(board);
    }

    pub fn clean(&self) {
        let mut board_list = self.board_list.lock().unwrap();
        board_list.clear();
    }
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_board(board_list: State<'_, BoardManager>) {
    let bit_board = Board::new();
    board_list.add(bit_board);
}

#[tauri::command]
fn handle_board(board_list: State<'_, BoardManager>) -> Vec<Vec<i32>> {

    let bit_board = board_list.current_board();
    let mut board = vec![vec![0;8];8];
    for y in 0..8 {
        for x in 0..8 {
            let mask: u64 = 1 << y * 8 + x;
            if bit_board.bit_board[0] & mask != 0 {
                board[y][x] = BLACK;
            }
            if bit_board.bit_board[1] & mask != 0 {
                board[y][x] = WHITE;
            }
        }
    }
    board
}

#[tauri::command]
fn handle_next_turn(board_list: State<'_, BoardManager>) -> i32 {
    let bit_board = board_list.current_board();
    bit_board.next_turn as i32
}


#[tauri::command]
fn put_piece_handle(board_list: State<'_, BoardManager>, y: i32, x: i32) {
    
    let ai_put = |board: &mut Board| {
        let depth_search = 64 - (board.bit_board[0].count_ones() + board.bit_board[1].count_ones());
        if depth_search <= 22 {
            eprintln!("turn count: {}", depth_search);
            board.put_piece(end_game_full_solver_nega_alpha(&board));
        } else {
            put_eval_one_simple(board);
        }
    };

    
    let mut board = board_list.current_board();
    if board.put_able() == 0 {
        board.next_turn ^= 1;
        board_list.add(board.clone());
        return;
    }
    if board.next_turn == Board::BLACK {
        //board.put_piece_from_coord(y, x);
        put_eval_zero_simple(&mut board);
        board_list.add(board.clone());
    } else {
        ai_put(&mut board);
        board_list.add(board.clone());
    }
    
}

#[tauri::command]
fn handle_undo(board_list: State<'_, BoardManager>) {
    board_list.undo();
}




fn create_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_submenu(submenu)
        .add_item(CustomMenuItem::new("hide", "Hide"));

    menu
}

fn on_main_menu_event(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        "hide" => event.window().hide().unwrap(),
        "quit" | "close" => event.window().close().unwrap(),
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,init_board, put_piece_handle, handle_board, handle_next_turn])
        .menu(create_menu())
        .on_menu_event(on_main_menu_event)
        .setup(|app| {
            // setupハンドラに、初期化処理が書けます。
            // App#manageメソッドでステート変数として管理するように登録できる。
            let board_manager = BoardManager::new();
            app.manage(board_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
