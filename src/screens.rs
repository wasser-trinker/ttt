use std::io;

use rand::Rng;

use crate::check_winner::check_winner;

use crate::inputs::is_field_free;
use crate::{
    board::print_board,
    inputs::{Spielfeld, get_input, is_draw},
};

pub fn start_screen() {
    println!("Drücke 1 um gegen einen Bot zu spielen");
    println!("\nDrücke 2 um mit deinem Freund zu spielen");

    let mut spiel_start_eingabe = String::new();

    get_input(&mut spiel_start_eingabe);

    let spiel_start_eingabe: u8 = match spiel_start_eingabe.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if spiel_start_eingabe == 1 {
        game_screen_solo();
    } else if spiel_start_eingabe == 2 {
        game_screen_duos()
    } else {
        println!("Bitte gebe eine gezeigte Option ein");
        start_screen()
    }
}

pub fn game_screen_duos() {
    let mut board = [Spielfeld::Leer; 9];

    let mut is_player_x = true;

    print_board(&board);

    loop {
        let mut feld_nummer = String::new();

        feld_nummer.clear();
        if is_player_x {
            println!("\nSpieler X wähle ein Feld (1-9)");
        } else {
            println!("\nSpieler O wähle ein Feld (1-9)");
        }

        io::stdin()
            .read_line(&mut feld_nummer)
            .expect("Bitte gib eine Nummer von 1-9 an!");

        let feld_nummer: usize = match feld_nummer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n Bitte nur das Feld eingeben.");
                continue;
            }
        };
        if feld_nummer > 9 || feld_nummer < 1 {
            continue;
        }

        if board[feld_nummer - 1] != Spielfeld::Leer {
            println!("Diese Spielfeld ist belegt!");
            println!("Bitte geben sie ein neues feld an.");
            continue;
        }

        if is_player_x {
            board[feld_nummer - 1] = Spielfeld::X;
            is_player_x = false;
        } else {
            board[feld_nummer - 1] = Spielfeld::O;
            is_player_x = true;
        }
        print_board(&board);

        match check_winner(&board) {
            // is_player_x ist der boolean der guckt ob Spiler X oder O dran ist
            Some(is_player_x) => {
                if is_player_x {
                    println!("Player X wins!");
                    break;
                } else {
                    println!("Player O wins!");
                    break;
                }
            }
            None => {}
        }
        if is_draw(&board) {
            // .iter() geht durch jedes einzelne array durch mit .all und guckt ob igendeins Leer ist. wenn ncht dann ist es unentschieden
            println!("Unentschieden!");
            break;
        }
    }

    end_screen()
}

pub fn game_screen_solo() {
    let mut board = [Spielfeld::Leer; 9];

    let mut is_player_x = true;

    print_board(&board);

    loop {
        let mut feld_nummer = String::new();

        feld_nummer.clear();
        if is_player_x {
            println!("\nSpieler X wähle ein Feld (1-9)");
            io::stdin()
                .read_line(&mut feld_nummer)
                .expect("Bitte gib eine Nummer von 1-9 an!");
        } else if is_player_x == false {
            println!("eine Nummer wird generiert");
            let feld_nummer: usize = rand::thread_rng().gen_range(1..=9);

            let free = is_field_free(&board, feld_nummer - 1);

            if free != false {
                if feld_nummer < 9 && feld_nummer >= 1 {
                    board[feld_nummer - 1] = Spielfeld::O;
                    is_player_x = true;
                    print_board(&board);
                    println!("{feld_nummer}");
                    match check_winner(&board) {
                        // is_player_x ist der boolean der guckt ob Spiler X oder O dran ist
                        Some(is_player_x) => {
                            if is_player_x {
                                println!("Player X wins!");
                                break;
                            } else {
                                println!("Player O wins!");
                                break;
                            }
                        }
                        None => {}
                    }
                    continue;
                }
            };
        }

        let feld_nummer: usize = match feld_nummer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if feld_nummer > 9 || feld_nummer < 1 {
            continue;
        }

        if board[feld_nummer - 1] != Spielfeld::Leer {
            println!("Diese Spielfeld ist belegt!");
            println!("Bitte geben sie ein neues feld an.");
            continue;
        }

        if is_player_x {
            board[feld_nummer - 1] = Spielfeld::X;
            is_player_x = false;
        }
        print_board(&board);
        match check_winner(&board) {
            // is_player_x ist der boolean der guckt ob Spiler X oder O dran ist
            Some(is_player_x) => {
                if is_player_x {
                    println!("Player X wins!");
                    break;
                } else {
                    println!("Player O wins!");
                    break;
                }
            }
            None => {}
        }
        if is_draw(&board) {
            // .iter() geht durch jedes einzelne array durch mit .all und guckt ob igendeins Leer ist. wenn ncht dann ist es unentschieden
            println!("Unentschieden!");
            break;
        }
    }

    end_screen()
}

pub fn end_screen() {
    println!("Nochmal Spielen?");
    println!("j/N");

    let mut wieder_spielen = String::new();

    io::stdin()
        .read_line(&mut wieder_spielen)
        .expect("Could not read line");

    if wieder_spielen.trim().to_lowercase() == "j" {
        // .trim() entfärnt die Whitespaces und die new lines (\n)
        start_screen()
    } else if wieder_spielen.trim() == "N" || wieder_spielen.trim() == "n" {
        std::process::exit(0)
    } else {
        println!("Bitte gebe eine gezeigte Option ein"); // wenn keiner der gezeigten angaben (J/N) nicht eingegeben wird wiederholt sich die frage udn die eingabe funktion
    }
}
