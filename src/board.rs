use crate::inputs::Spielfeld;

pub fn print_board(board: &[Spielfeld; 9]) {
    // das spielfeld (board)
    for feldy in 0..=2 {
        for feldx in 0..=2 {
            match board[feldy * 3 + feldx] {
                Spielfeld::X => print!(" X "),
                Spielfeld::O => print!(" O "),
                Spielfeld::Leer => print!(" _ "),
            }
        }
        println!();
    }
}
