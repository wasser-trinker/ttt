use std::io;

#[derive(Clone, Copy, PartialEq, Debug)]

pub enum Spielfeld {
    X,
    O,
    Leer,
}

pub fn get_input(spiel_start_eingabe: &mut String) {
    io::stdin()
        .read_line(spiel_start_eingabe)
        .expect("Could not read line");
}

// pub fn check_input(input: &str, options: &[&str]) -> bool {
//     options.contains(&input)
// }

pub fn is_draw(board: &[Spielfeld; 9]) -> bool {
    board.iter().all(|feld| *feld != Spielfeld::Leer)
}

pub fn is_field_free(board: &[Spielfeld; 9], index: usize) -> bool {
    board[index] == Spielfeld::Leer
}
