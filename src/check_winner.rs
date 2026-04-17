use crate::inputs::Spielfeld;

pub fn check_winner(board: &[Spielfeld; 9]) -> Option<bool> {
    if board[0] == board[1] && board[1] == board[2] && board[1] == Spielfeld::X {
        // 1.
        Some(true)
    } else if board[0] == board[1] && board[1] == board[2] && board[1] == Spielfeld::O {
        Some(false)
    } else if board[0] == board[3] && board[3] == board[6] && board[3] == Spielfeld::X {
        // 2.
        Some(true)
    } else if board[0] == board[3] && board[3] == board[6] && board[3] == Spielfeld::O {
        Some(false)
    } else if board[0] == board[4] && board[4] == board[8] && board[4] == Spielfeld::X {
        // 3.
        Some(true)
    } else if board[0] == board[4] && board[4] == board[8] && board[4] == Spielfeld::O {
        Some(false)
    } else if board[1] == board[4] && board[4] == board[7] && board[4] == Spielfeld::X {
        // 4.
        return Some(true);
    } else if board[1] == board[4] && board[4] == board[7] && board[4] == Spielfeld::O {
        return Some(false);
    } else if board[2] == board[4] && board[4] == board[6] && board[4] == Spielfeld::X {
        // 5.
        return Some(true);
    } else if board[2] == board[4] && board[4] == board[6] && board[4] == Spielfeld::O {
        return Some(false);
    } else if board[2] == board[5] && board[5] == board[8] && board[5] == Spielfeld::X {
        // 6.
        return Some(true);
    } else if board[2] == board[5] && board[5] == board[8] && board[5] == Spielfeld::O {
        return Some(false);
    } else if board[3] == board[4] && board[4] == board[5] && board[4] == Spielfeld::X {
        // 7.
        return Some(true);
    } else if board[3] == board[4] && board[4] == board[5] && board[4] == Spielfeld::O {
        return Some(false);
    } else if board[6] == board[7] && board[7] == board[8] && board[7] == Spielfeld::X {
        // 8.
        return Some(true);
    } else if board[6] == board[7] && board[7] == board[8] && board[7] == Spielfeld::O {
        return Some(false);
    } else {
        None
    }
}
