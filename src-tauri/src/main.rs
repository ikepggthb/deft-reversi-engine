// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Menu, CustomMenuItem, Submenu, WindowMenuEvent, Wry};

mod board;
use board::*;
mod ai;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init_board() -> Vec<Vec<i32>> {
    let bit_board = Board::new();
    let mut board = vec![vec![0;8];8];

    for y in 0..8 {
        for x in 0..8 {
            let mask: u64 = 1 << y * 8 + x;
            if bit_board.bit_board[0] & mask != 0 {
                board[y][x] = 1;
            }
            if bit_board.bit_board[1] & mask != 0 {
                board[y][x] = 2;
            }
        }
    }
    
    board
}


#[tauri::command]
fn put_piece_handle(y: i32, x: i32) -> Vec<Vec<i32>> {
    let mut bit_board = Board::new();
    bit_board.put_piece_from_coord(y, x);
    let mut board = vec![vec![0;8];8];

    for y in 0..8 {
        for x in 0..8 {
            let mask: u64 = 1 << y * 8 + x;
            if bit_board.bit_board[0] & mask != 0 {
                board[y][x] = 1;
            }
            if bit_board.bit_board[1] & mask != 0 {
                board[y][x] = 2;
            }
        }
    }
    
    board
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
        .invoke_handler(tauri::generate_handler![greet,init_board, put_piece_handle])
        .menu(create_menu())
        .on_menu_event(on_main_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
